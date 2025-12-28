macro_rules! deps {
    () => {
        TestDB!();
        Query!();
        AssocItemId!();
        ImportMap!();
        ItemInNs!();
        AssocSearchMode!();
        DefDatabase!();
        ModuleDefId!();
        ImportInfo!();
        ItemContainerId!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use base_db :: RootQueryDb ; use expect_test :: { Expect , expect } ; use test_fixture :: WithFixture ; use crate :: { ItemContainerId , Lookup , nameres :: assoc :: TraitItems , test_db :: TestDB } ; use super :: * ; impl ImportMap { fn fmt_for_test (& self , db : & dyn DefDatabase) -> String { let mut importable_paths : Vec < _ > = self . item_to_info_map . iter () . flat_map (| (item , (info , _)) | info . iter () . map (move | info | (item , info))) . map (| (item , info) | { let path = render_path (db , info) ; let ns = match item { ItemInNs :: Types (_) => "t" , ItemInNs :: Values (_) => "v" , ItemInNs :: Macros (_) => "m" , } ; format ! ("- {path} ({ns})") }) . collect () ; importable_paths . sort () ; importable_paths . join ("\n") } } fn check_search (# [rust_analyzer :: rust_fixture] ra_fixture : & str , crate_name : & str , query : Query , expect : Expect ,) { let db = TestDB :: with_files (ra_fixture) ; let all_crates = db . all_crates () ; let krate = all_crates . iter () . copied () . find (| & krate | { krate . extra_data (& db) . display_name . as_ref () . is_some_and (| it | it . crate_name () . as_str () == crate_name) }) . expect ("could not find crate") ; let actual = search_dependencies (& db , krate , & query) . into_iter () . filter_map (| (dependency , _) | { let dependency_krate = dependency . krate (& db) ? ; let dependency_imports = db . import_map (dependency_krate) ; let (path , mark) = match assoc_item_path (& db , & dependency_imports , dependency) { Some (assoc_item_path) => (assoc_item_path , "a") , None => (render_path (& db , & dependency_imports . import_info_for (dependency) ? [0]) , match dependency { ItemInNs :: Types (ModuleDefId :: FunctionId (_)) | ItemInNs :: Values (ModuleDefId :: FunctionId (_)) => "f" , ItemInNs :: Types (_) => "t" , ItemInNs :: Values (_) => "v" , ItemInNs :: Macros (_) => "m" , } ,) , } ; Some (format ! ("{}::{} ({})\n" , dependency_krate . extra_data (& db) . display_name . as_ref () ?, path , mark)) }) . sorted () . collect :: < String > () ; expect . assert_eq (& actual) } fn assoc_item_path (db : & dyn DefDatabase , dependency_imports : & ImportMap , dependency : ItemInNs ,) -> Option < String > { let (dependency_assoc_item_id , container) = match dependency . as_module_def_id () ? { ModuleDefId :: FunctionId (id) => (AssocItemId :: from (id) , id . lookup (db) . container) , ModuleDefId :: ConstId (id) => (AssocItemId :: from (id) , id . lookup (db) . container) , ModuleDefId :: TypeAliasId (id) => (AssocItemId :: from (id) , id . lookup (db) . container) , _ => return None , } ; let ItemContainerId :: TraitId (trait_id) = container else { return None ; } ; let trait_info = dependency_imports . import_info_for (ItemInNs :: Types (trait_id . into ())) ? ; let trait_items = TraitItems :: query (db , trait_id) ; let (assoc_item_name , _) = trait_items . items . iter () . find (| (_ , assoc_item_id) | & dependency_assoc_item_id == assoc_item_id) ? ; Some (format ! ("{}::{}" , render_path (db , & trait_info [0]) , assoc_item_name . display (db , Edition :: CURRENT))) } fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let db = TestDB :: with_files (ra_fixture) ; let all_crates = db . all_crates () ; let actual = all_crates . iter () . copied () . filter_map (| krate | { let cdata = & krate . extra_data (& db) ; let name = cdata . display_name . as_ref () ? ; let map = db . import_map (krate) ; Some (format ! ("{name}:\n{}\n" , map . fmt_for_test (& db))) }) . sorted () . collect :: < String > () ; expect . assert_eq (& actual) } fn render_path (db : & dyn DefDatabase , info : & ImportInfo) -> String { let mut module = info . container ; let mut segments = vec ! [& info . name] ; let def_map = module . def_map (db) ; assert ! (def_map . block_id () . is_none () , "block local items should not be in `ImportMap`") ; while let Some (parent) = module . containing_module (db) { let parent_data = & def_map [parent . local_id] ; let (name , _) = parent_data . children . iter () . find (| (_ , id) | * * id == module . local_id) . unwrap () ; segments . push (name) ; module = parent ; } segments . iter () . rev () . map (| it | it . display (db , Edition :: CURRENT)) . join ("::") } # [test] fn smoke () { check (r"
            //- /main.rs crate:main deps:lib

            mod private {
                pub use lib::Pub;
                pub struct InPrivateModule;
            }

            pub mod publ1 {
                use lib::Pub;
            }

            pub mod real_pub {
                pub use lib::Pub;
            }
            pub mod real_pu2 { // same path length as above
                pub use lib::Pub;
            }

            //- /lib.rs crate:lib
            pub struct Pub {}
            pub struct Pub2; // t + v
            struct Priv;
        " , expect ! [[r#"
                lib:
                - Pub (t)
                - Pub2 (t)
                - Pub2 (v)
                main:
                - publ1 (t)
                - real_pu2 (t)
                - real_pu2::Pub (t)
                - real_pub (t)
                - real_pub::Pub (t)
            "#]] ,) ; } # [test] fn prefers_shortest_path () { check (r"
            //- /main.rs crate:main

            pub mod sub {
                pub mod subsub {
                    pub struct Def {}
                }

                pub use super::sub::subsub::Def;
            }
        " , expect ! [[r#"
                main:
                - sub (t)
                - sub::Def (t)
                - sub::subsub (t)
                - sub::subsub::Def (t)
            "#]] ,) ; } # [test] fn type_reexport_cross_crate () { check (r"
            //- /main.rs crate:main deps:lib
            pub mod m {
                pub use lib::S;
            }
            //- /lib.rs crate:lib
            pub struct S;
        " , expect ! [[r#"
                lib:
                - S (t)
                - S (v)
                main:
                - m (t)
                - m::S (t)
                - m::S (v)
            "#]] ,) ; } # [test] fn macro_reexport () { check (r"
            //- /main.rs crate:main deps:lib
            pub mod m {
                pub use lib::pub_macro;
            }
            //- /lib.rs crate:lib
            #[macro_export]
            macro_rules! pub_macro {
                () => {};
            }
        " , expect ! [[r#"
                lib:
                - pub_macro (m)
                main:
                - m (t)
                - m::pub_macro (m)
            "#]] ,) ; } # [test] fn module_reexport () { check (r"
            //- /main.rs crate:main deps:lib
            pub use lib::module as reexported_module;
            //- /lib.rs crate:lib
            pub mod module {
                pub struct S;
            }
        " , expect ! [[r#"
                lib:
                - module (t)
                - module::S (t)
                - module::S (v)
                main:
                - module::S (t)
                - module::S (v)
                - reexported_module (t)
            "#]] ,) ; } # [test] fn cyclic_module_reexport () { check (r"
            //- /lib.rs crate:lib
            pub mod module {
                pub struct S;
                pub use super::sub::*;
            }

            pub mod sub {
                pub use super::module;
            }
        " , expect ! [[r#"
                lib:
                - module (t)
                - module::S (t)
                - module::S (v)
                - module::module (t)
                - sub (t)
                - sub::module (t)
            "#]] ,) ; } # [test] fn private_macro () { check (r"
            //- /lib.rs crate:lib
            macro_rules! private_macro {
                () => {};
            }
        " , expect ! [[r#"
                lib:

            "#]] ,) ; } # [test] fn namespacing () { check (r"
            //- /lib.rs crate:lib
            pub struct Thing;     // t + v
            #[macro_export]
            macro_rules! Thing {  // m
                () => {};
            }
        " , expect ! [[r#"
                lib:
                - Thing (m)
                - Thing (t)
                - Thing (v)
            "#]] ,) ; check (r"
            //- /lib.rs crate:lib
            pub mod Thing {}      // t
            #[macro_export]
            macro_rules! Thing {  // m
                () => {};
            }
        " , expect ! [[r#"
                lib:
                - Thing (m)
                - Thing (t)
            "#]] ,) ; } # [test] fn fuzzy_import_trait_and_assoc_items () { cov_mark :: check ! (type_aliases_ignored) ; let ra_fixture = r#"
        //- /main.rs crate:main deps:dep
        //- /dep.rs crate:dep
        pub mod fmt {
            pub trait Display {
                type FmtTypeAlias;
                const FMT_CONST: bool;

                fn format_function();
                fn format_method(&self);
            }
        }
    "# ; check_search (ra_fixture , "main" , Query :: new ("fmt" . to_owned ()) . fuzzy () , expect ! [[r#"
                dep::fmt (t)
                dep::fmt::Display::FMT_CONST (a)
                dep::fmt::Display::format_function (a)
                dep::fmt::Display::format_method (a)
            "#]] ,) ; } # [test] fn assoc_items_filtering () { let ra_fixture = r#"
        //- /main.rs crate:main deps:dep
        //- /dep.rs crate:dep
        pub mod fmt {
            pub trait Display {
                type FmtTypeAlias;
                const FMT_CONST: bool;

                fn format_function();
                fn format_method(&self);
            }
        }
    "# ; check_search (ra_fixture , "main" , Query :: new ("fmt" . to_owned ()) . fuzzy () . assoc_search_mode (AssocSearchMode :: AssocItemsOnly) , expect ! [[r#"
                dep::fmt::Display::FMT_CONST (a)
                dep::fmt::Display::format_function (a)
                dep::fmt::Display::format_method (a)
            "#]] ,) ; check_search (ra_fixture , "main" , Query :: new ("fmt" . to_owned ()) . fuzzy () . assoc_search_mode (AssocSearchMode :: Exclude) , expect ! [[r#"
                dep::fmt (t)
            "#]] ,) ; } # [test] fn search_mode () { let ra_fixture = r#"
//- /main.rs crate:main deps:dep
//- /dep.rs crate:dep deps:tdep
use tdep::fmt as fmt_dep;
pub mod fmt {
    pub trait Display {
        fn fmt();
    }
}
#[macro_export]
macro_rules! Fmt {
    () => {};
}
pub struct Fmt;

pub fn format() {}
pub fn no() {}

//- /tdep.rs crate:tdep
pub mod fmt {
    pub struct NotImportableFromMain;
}
"# ; check_search (ra_fixture , "main" , Query :: new ("fmt" . to_owned ()) . fuzzy () , expect ! [[r#"
                dep::Fmt (m)
                dep::Fmt (t)
                dep::Fmt (v)
                dep::fmt (t)
                dep::fmt::Display::fmt (a)
                dep::format (f)
            "#]] ,) ; check_search (ra_fixture , "main" , Query :: new ("fmt" . to_owned ()) , expect ! [[r#"
                dep::Fmt (m)
                dep::Fmt (t)
                dep::Fmt (v)
                dep::fmt (t)
                dep::fmt::Display::fmt (a)
            "#]] ,) ; } # [test] fn name_only () { let ra_fixture = r#"
            //- /main.rs crate:main deps:dep
            //- /dep.rs crate:dep deps:tdep
            use tdep::fmt as fmt_dep;
            pub mod fmt {
                pub trait Display {
                    fn fmt();
                }
            }
            #[macro_export]
            macro_rules! Fmt {
                () => {};
            }
            pub struct Fmt;

            pub fn format() {}
            pub fn no() {}

            //- /tdep.rs crate:tdep
            pub mod fmt {
                pub struct NotImportableFromMain;
            }
        "# ; check_search (ra_fixture , "main" , Query :: new ("fmt" . to_owned ()) , expect ! [[r#"
                dep::Fmt (m)
                dep::Fmt (t)
                dep::Fmt (v)
                dep::fmt (t)
                dep::fmt::Display::fmt (a)
            "#]] ,) ; } # [test] fn search_casing () { let ra_fixture = r#"
            //- /main.rs crate:main deps:dep
            //- /dep.rs crate:dep

            pub struct fmt;
            pub struct FMT;
        "# ; check_search (ra_fixture , "main" , Query :: new ("FMT" . to_owned ()) , expect ! [[r#"
                dep::FMT (t)
                dep::FMT (v)
                dep::fmt (t)
                dep::fmt (v)
            "#]] ,) ; check_search (ra_fixture , "main" , Query :: new ("FMT" . to_owned ()) . case_sensitive () , expect ! [[r#"
                dep::FMT (t)
                dep::FMT (v)
            "#]] ,) ; } # [test] fn unicode_fn_name () { let ra_fixture = r#"
            //- /main.rs crate:main deps:dep
            //- /dep.rs crate:dep
            pub fn あい() {}
        "# ; check_search (ra_fixture , "main" , Query :: new ("あ" . to_owned ()) . fuzzy () , expect ! [[r#"
            dep::あい (f)
        "#]] ,) ; } }
    };
}

tests!();