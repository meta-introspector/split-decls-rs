// Generated macro for tests (module)
macro_rules! Depcrate_find_pathtests {
() => {
// Module: crate::find_path
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use expect_test :: { Expect , expect } ; use hir_expand :: db :: ExpandDatabase ; use itertools :: Itertools ; use span :: Edition ; use stdx :: format_to ; use syntax :: ast :: AstNode ; use test_fixture :: WithFixture ; use crate :: test_db :: TestDB ; use super :: * ; # [doc = " `code` needs to contain a cursor marker; checks that `find_path` for the"] # [doc = " item the `path` refers to returns that same path when called from the"] # [doc = " module the cursor is in."] # [track_caller] fn check_found_path_ (# [rust_analyzer :: rust_fixture] ra_fixture : & str , path : & str , prefer_prelude : bool , prefer_absolute : bool , prefer_no_std : bool , allow_unstable : bool , expect : Expect ,) { let (db , pos) = TestDB :: with_position (ra_fixture) ; let module = db . module_at_position (pos) ; let parsed_path_file = syntax :: SourceFile :: parse (& format ! ("use {path};") , span :: Edition :: CURRENT) ; let ast_path = parsed_path_file . syntax_node () . descendants () . find_map (syntax :: ast :: Path :: cast) . unwrap () ; let mod_path = ModPath :: from_src (& db , ast_path , & mut | range | { db . span_map (pos . file_id . into ()) . as_ref () . span_for_range (range) . ctx }) . unwrap () ; let (def_map , local_def_map) = module . local_def_map (& db) ; let resolved = def_map . resolve_path (local_def_map , & db , module . local_id , & mod_path , crate :: item_scope :: BuiltinShadowMode :: Module , None ,) . 0 ; let resolved = resolved . take_types () . map (ItemInNs :: Types) . or_else (| | resolved . take_values () . map (ItemInNs :: Values)) . expect ("path does not resolve to a type or value") ; let mut res = String :: new () ; for (prefix , ignore_local_imports) in [PrefixKind :: Plain , PrefixKind :: ByCrate , PrefixKind :: BySelf] . into_iter () . cartesian_product ([false , true]) { let found_path = find_path (& db , resolved , module , prefix , ignore_local_imports , FindPathConfig { prefer_no_std , prefer_prelude , prefer_absolute , allow_unstable } ,) ; format_to ! (res , "{:7}(imports {}): {}\n" , format ! ("{:?}" , prefix) , if ignore_local_imports { '✖' } else { '✔' } , found_path . map_or_else (|| "<unresolvable>" . to_owned () , | it | it . display (& db , Edition :: CURRENT) . to_string ()) ,) ; } expect . assert_eq (& res) ; } fn check_found_path (# [rust_analyzer :: rust_fixture] ra_fixture : & str , path : & str , expect : Expect ,) { check_found_path_ (ra_fixture , path , false , false , false , false , expect) ; } fn check_found_path_prelude (# [rust_analyzer :: rust_fixture] ra_fixture : & str , path : & str , expect : Expect ,) { check_found_path_ (ra_fixture , path , true , false , false , false , expect) ; } fn check_found_path_absolute (# [rust_analyzer :: rust_fixture] ra_fixture : & str , path : & str , expect : Expect ,) { check_found_path_ (ra_fixture , path , false , true , false , false , expect) ; } fn check_found_path_prefer_no_std (# [rust_analyzer :: rust_fixture] ra_fixture : & str , path : & str , expect : Expect ,) { check_found_path_ (ra_fixture , path , false , false , true , false , expect) ; } fn check_found_path_prefer_no_std_allow_unstable (# [rust_analyzer :: rust_fixture] ra_fixture : & str , path : & str , expect : Expect ,) { check_found_path_ (ra_fixture , path , false , false , true , true , expect) ; } # [test] fn same_module () { check_found_path (r#"
struct S;
$0
        "# , "S" , expect ! [[r#"
                Plain  (imports ✔): S
                Plain  (imports ✖): S
                ByCrate(imports ✔): crate::S
                ByCrate(imports ✖): crate::S
                BySelf (imports ✔): self::S
                BySelf (imports ✖): self::S
            "#]] ,) ; } # [test] fn enum_variant () { check_found_path (r#"
enum E { A }
$0
        "# , "E::A" , expect ! [[r#"
                Plain  (imports ✔): E::A
                Plain  (imports ✖): E::A
                ByCrate(imports ✔): crate::E::A
                ByCrate(imports ✖): crate::E::A
                BySelf (imports ✔): self::E::A
                BySelf (imports ✖): self::E::A
            "#]] ,) ; } # [test] fn sub_module () { check_found_path (r#"
mod foo {
    pub struct S;
}
$0
        "# , "foo::S" , expect ! [[r#"
                Plain  (imports ✔): foo::S
                Plain  (imports ✖): foo::S
                ByCrate(imports ✔): crate::foo::S
                ByCrate(imports ✖): crate::foo::S
                BySelf (imports ✔): self::foo::S
                BySelf (imports ✖): self::foo::S
            "#]] ,) ; } # [test] fn super_module () { check_found_path (r#"
//- /main.rs
mod foo;
//- /foo.rs
mod bar;
struct S;
//- /foo/bar.rs
$0
        "# , "super::S" , expect ! [[r#"
                Plain  (imports ✔): super::S
                Plain  (imports ✖): super::S
                ByCrate(imports ✔): crate::foo::S
                ByCrate(imports ✖): crate::foo::S
                BySelf (imports ✔): super::S
                BySelf (imports ✖): super::S
            "#]] ,) ; } # [test] fn self_module () { check_found_path (r#"
//- /main.rs
mod foo;
//- /foo.rs
$0
        "# , "self" , expect ! [[r#"
                Plain  (imports ✔): self
                Plain  (imports ✖): self
                ByCrate(imports ✔): crate::foo
                ByCrate(imports ✖): crate::foo
                BySelf (imports ✔): self
                BySelf (imports ✖): self
            "#]] ,) ; } # [test] fn crate_root () { check_found_path (r#"
//- /main.rs
mod foo;
//- /foo.rs
$0
        "# , "crate" , expect ! [[r#"
                Plain  (imports ✔): crate
                Plain  (imports ✖): crate
                ByCrate(imports ✔): crate
                ByCrate(imports ✖): crate
                BySelf (imports ✔): crate
                BySelf (imports ✖): crate
            "#]] ,) ; } # [test] fn same_crate () { check_found_path (r#"
//- /main.rs
mod foo;
struct S;
//- /foo.rs
$0
        "# , "crate::S" , expect ! [[r#"
                Plain  (imports ✔): crate::S
                Plain  (imports ✖): crate::S
                ByCrate(imports ✔): crate::S
                ByCrate(imports ✖): crate::S
                BySelf (imports ✔): crate::S
                BySelf (imports ✖): crate::S
            "#]] ,) ; } # [test] fn different_crate () { check_found_path (r#"
//- /main.rs crate:main deps:std
$0
//- /std.rs crate:std
pub struct S;
        "# , "std::S" , expect ! [[r#"
                Plain  (imports ✔): std::S
                Plain  (imports ✖): std::S
                ByCrate(imports ✔): std::S
                ByCrate(imports ✖): std::S
                BySelf (imports ✔): std::S
                BySelf (imports ✖): std::S
            "#]] ,) ; } # [test] fn different_crate_renamed () { check_found_path (r#"
//- /main.rs crate:main deps:std
extern crate std as std_renamed;
$0
//- /std.rs crate:std
pub struct S;
        "# , "std_renamed::S" , expect ! [[r#"
                Plain  (imports ✔): std_renamed::S
                Plain  (imports ✖): std_renamed::S
                ByCrate(imports ✔): std_renamed::S
                ByCrate(imports ✖): std_renamed::S
                BySelf (imports ✔): std_renamed::S
                BySelf (imports ✖): std_renamed::S
            "#]] ,) ; } # [test] fn partially_imported () { cov_mark :: check ! (partially_imported) ; check_found_path (r#"
//- /main.rs crate:main deps:syntax

use syntax::ast;
$0

//- /lib.rs crate:syntax
pub mod ast {
    pub enum ModuleItem {
        A, B, C,
    }
}
        "# , "syntax::ast::ModuleItem" , expect ! [[r#"
                Plain  (imports ✔): ast::ModuleItem
                Plain  (imports ✖): syntax::ast::ModuleItem
                ByCrate(imports ✔): crate::ast::ModuleItem
                ByCrate(imports ✖): syntax::ast::ModuleItem
                BySelf (imports ✔): self::ast::ModuleItem
                BySelf (imports ✖): syntax::ast::ModuleItem
            "#]] ,) ; check_found_path (r#"
//- /main.rs crate:main deps:syntax
$0

//- /lib.rs crate:syntax
pub mod ast {
    pub enum ModuleItem {
        A, B, C,
    }
}
        "# , "syntax::ast::ModuleItem" , expect ! [[r#"
                Plain  (imports ✔): syntax::ast::ModuleItem
                Plain  (imports ✖): syntax::ast::ModuleItem
                ByCrate(imports ✔): syntax::ast::ModuleItem
                ByCrate(imports ✖): syntax::ast::ModuleItem
                BySelf (imports ✔): syntax::ast::ModuleItem
                BySelf (imports ✖): syntax::ast::ModuleItem
            "#]] ,) ; } # [test] fn partially_imported_with_prefer_absolute () { cov_mark :: check ! (partially_imported) ; check_found_path_absolute (r#"
//- /main.rs crate:main deps:syntax

use syntax::ast;
$0

//- /lib.rs crate:syntax
pub mod ast {
    pub enum ModuleItem {
        A, B, C,
    }
}
        "# , "syntax::ast::ModuleItem" , expect ! [[r#"
                Plain  (imports ✔): ast::ModuleItem
                Plain  (imports ✖): ::syntax::ast::ModuleItem
                ByCrate(imports ✔): crate::ast::ModuleItem
                ByCrate(imports ✖): ::syntax::ast::ModuleItem
                BySelf (imports ✔): self::ast::ModuleItem
                BySelf (imports ✖): ::syntax::ast::ModuleItem
            "#]] ,) ; } # [test] fn same_crate_reexport () { check_found_path (r#"
mod bar {
    mod foo { pub(crate) struct S; }
    pub(crate) use foo::*;
}
$0
        "# , "bar::S" , expect ! [[r#"
                Plain  (imports ✔): bar::S
                Plain  (imports ✖): bar::S
                ByCrate(imports ✔): crate::bar::S
                ByCrate(imports ✖): crate::bar::S
                BySelf (imports ✔): self::bar::S
                BySelf (imports ✖): self::bar::S
            "#]] ,) ; } # [test] fn same_crate_reexport_rename () { check_found_path (r#"
mod bar {
    mod foo { pub(crate) struct S; }
    pub(crate) use foo::S as U;
}
$0
        "# , "bar::U" , expect ! [[r#"
                Plain  (imports ✔): bar::U
                Plain  (imports ✖): bar::U
                ByCrate(imports ✔): crate::bar::U
                ByCrate(imports ✖): crate::bar::U
                BySelf (imports ✔): self::bar::U
                BySelf (imports ✖): self::bar::U
            "#]] ,) ; } # [test] fn different_crate_reexport () { check_found_path (r#"
//- /main.rs crate:main deps:std
$0
//- /std.rs crate:std deps:core
pub use core::S;
//- /core.rs crate:core
pub struct S;
        "# , "std::S" , expect ! [[r#"
                Plain  (imports ✔): std::S
                Plain  (imports ✖): std::S
                ByCrate(imports ✔): std::S
                ByCrate(imports ✖): std::S
                BySelf (imports ✔): std::S
                BySelf (imports ✖): std::S
            "#]] ,) ; } # [test] fn prelude () { check_found_path (r#"
//- /main.rs edition:2018 crate:main deps:std
$0
//- /std.rs crate:std
pub mod prelude {
    pub mod rust_2018 {
        pub struct S;
    }
}
        "# , "S" , expect ! [[r#"
                Plain  (imports ✔): S
                Plain  (imports ✖): S
                ByCrate(imports ✔): S
                ByCrate(imports ✖): S
                BySelf (imports ✔): S
                BySelf (imports ✖): S
            "#]] ,) ; } # [test] fn shadowed_prelude () { check_found_path (r#"
//- /main.rs crate:main deps:std
struct S;
$0
//- /std.rs crate:std
pub mod prelude {
    pub mod rust_2018 {
        pub struct S;
    }
}
"# , "std::prelude::rust_2018::S" , expect ! [[r#"
                Plain  (imports ✔): std::prelude::rust_2018::S
                Plain  (imports ✖): std::prelude::rust_2018::S
                ByCrate(imports ✔): std::prelude::rust_2018::S
                ByCrate(imports ✖): std::prelude::rust_2018::S
                BySelf (imports ✔): std::prelude::rust_2018::S
                BySelf (imports ✖): std::prelude::rust_2018::S
            "#]] ,) ; } # [test] fn imported_prelude () { check_found_path (r#"
//- /main.rs edition:2018 crate:main deps:std
use S;
$0
//- /std.rs crate:std
pub mod prelude {
    pub mod rust_2018 {
        pub struct S;
    }
}
"# , "S" , expect ! [[r#"
                Plain  (imports ✔): S
                Plain  (imports ✖): S
                ByCrate(imports ✔): crate::S
                ByCrate(imports ✖): S
                BySelf (imports ✔): self::S
                BySelf (imports ✖): S
            "#]] ,) ; } # [test] fn enum_variant_from_prelude () { let code = r#"
//- /main.rs edition:2018 crate:main deps:std
$0
//- /std.rs crate:std
pub mod prelude {
    pub mod rust_2018 {
        pub enum Option<T> { Some(T), None }
        pub use Option::*;
    }
}
        "# ; check_found_path (code , "None" , expect ! [[r#"
                Plain  (imports ✔): None
                Plain  (imports ✖): None
                ByCrate(imports ✔): None
                ByCrate(imports ✖): None
                BySelf (imports ✔): None
                BySelf (imports ✖): None
            "#]] ,) ; check_found_path (code , "Some" , expect ! [[r#"
                Plain  (imports ✔): Some
                Plain  (imports ✖): Some
                ByCrate(imports ✔): Some
                ByCrate(imports ✖): Some
                BySelf (imports ✔): Some
                BySelf (imports ✖): Some
            "#]] ,) ; } # [test] fn shortest_path () { check_found_path (r#"
//- /main.rs
pub mod foo;
pub mod baz;
struct S;
$0
//- /foo.rs
pub mod bar { pub struct S; }
//- /baz.rs
pub use crate::foo::bar::S;
        "# , "baz::S" , expect ! [[r#"
                Plain  (imports ✔): baz::S
                Plain  (imports ✖): baz::S
                ByCrate(imports ✔): crate::baz::S
                ByCrate(imports ✖): crate::baz::S
                BySelf (imports ✔): self::baz::S
                BySelf (imports ✖): self::baz::S
            "#]] ,) ; } # [test] fn discount_private_imports () { cov_mark :: check ! (discount_private_imports) ; check_found_path (r#"
//- /main.rs
mod foo;
pub mod bar { pub struct S; }
use bar::S;
//- /foo.rs
$0
        "# , "crate::bar::S" , expect ! [[r#"
                Plain  (imports ✔): crate::bar::S
                Plain  (imports ✖): crate::bar::S
                ByCrate(imports ✔): crate::bar::S
                ByCrate(imports ✖): crate::bar::S
                BySelf (imports ✔): crate::bar::S
                BySelf (imports ✖): crate::bar::S
            "#]] ,) ; } # [test] fn explicit_private_imports_crate () { check_found_path (r#"
//- /main.rs
mod foo;
pub mod bar { pub struct S; }
pub(crate) use bar::S;
//- /foo.rs
$0
        "# , "crate::S" , expect ! [[r#"
                Plain  (imports ✔): crate::S
                Plain  (imports ✖): crate::S
                ByCrate(imports ✔): crate::S
                ByCrate(imports ✖): crate::S
                BySelf (imports ✔): crate::S
                BySelf (imports ✖): crate::S
            "#]] ,) ; } # [test] fn explicit_private_imports () { cov_mark :: check ! (explicit_private_imports) ; check_found_path (r#"
//- /main.rs
pub mod bar {
    mod foo;
    pub mod baz { pub struct S; }
    pub(self) use baz::S;
}

//- /bar/foo.rs
$0
        "# , "super::S" , expect ! [[r#"
                Plain  (imports ✔): super::S
                Plain  (imports ✖): super::S
                ByCrate(imports ✔): crate::bar::S
                ByCrate(imports ✖): crate::bar::S
                BySelf (imports ✔): super::S
                BySelf (imports ✖): super::S
            "#]] ,) ; } # [test] fn import_cycle () { check_found_path (r#"
//- /main.rs
pub mod foo;
pub mod bar;
pub mod baz;
//- /bar.rs
$0
//- /foo.rs
pub use super::baz;
pub struct S;
//- /baz.rs
pub use super::foo;
        "# , "crate::foo::S" , expect ! [[r#"
                Plain  (imports ✔): crate::foo::S
                Plain  (imports ✖): crate::foo::S
                ByCrate(imports ✔): crate::foo::S
                ByCrate(imports ✖): crate::foo::S
                BySelf (imports ✔): crate::foo::S
                BySelf (imports ✖): crate::foo::S
            "#]] ,) ; } # [test] fn prefer_std_paths_over_alloc () { check_found_path (r#"
//- /main.rs crate:main deps:alloc,std
$0

//- /std.rs crate:std deps:alloc
pub mod sync {
    pub use alloc::sync::Arc;
}

//- /zzz.rs crate:alloc
pub mod sync {
    pub struct Arc;
}
        "# , "std::sync::Arc" , expect ! [[r#"
                Plain  (imports ✔): std::sync::Arc
                Plain  (imports ✖): std::sync::Arc
                ByCrate(imports ✔): std::sync::Arc
                ByCrate(imports ✖): std::sync::Arc
                BySelf (imports ✔): std::sync::Arc
                BySelf (imports ✖): std::sync::Arc
            "#]] ,) ; } # [test] fn prefer_core_paths_over_std_for_mod_reexport () { check_found_path_prefer_no_std (r#"
//- /main.rs crate:main deps:core,std

$0

//- /stdlib.rs crate:std deps:core

pub use core::pin;

//- /corelib.rs crate:core

pub mod pin {
    pub struct Pin;
}
            "# , "std::pin::Pin" , expect ! [[r#"
                Plain  (imports ✔): core::pin::Pin
                Plain  (imports ✖): core::pin::Pin
                ByCrate(imports ✔): core::pin::Pin
                ByCrate(imports ✖): core::pin::Pin
                BySelf (imports ✔): core::pin::Pin
                BySelf (imports ✖): core::pin::Pin
            "#]] ,) ; } # [test] fn prefer_core_paths_over_std () { check_found_path_prefer_no_std (r#"
//- /main.rs crate:main deps:core,std

$0

//- /std.rs crate:std deps:core

pub mod fmt {
    pub use core::fmt::Error;
}

//- /zzz.rs crate:core

pub mod fmt {
    pub struct Error;
}
        "# , "core::fmt::Error" , expect ! [[r#"
                Plain  (imports ✔): core::fmt::Error
                Plain  (imports ✖): core::fmt::Error
                ByCrate(imports ✔): core::fmt::Error
                ByCrate(imports ✖): core::fmt::Error
                BySelf (imports ✔): core::fmt::Error
                BySelf (imports ✖): core::fmt::Error
            "#]] ,) ; check_found_path (r#"
//- /main.rs crate:main deps:core,std
#![no_std]

$0

//- /std.rs crate:std deps:core

pub mod fmt {
    pub use core::fmt::Error;
}

//- /zzz.rs crate:core

pub mod fmt {
    pub struct Error;
}
        "# , "core::fmt::Error" , expect ! [[r#"
                Plain  (imports ✔): core::fmt::Error
                Plain  (imports ✖): core::fmt::Error
                ByCrate(imports ✔): core::fmt::Error
                ByCrate(imports ✖): core::fmt::Error
                BySelf (imports ✔): core::fmt::Error
                BySelf (imports ✖): core::fmt::Error
            "#]] ,) ; check_found_path (r#"
//- /main.rs crate:main deps:core,std
#![cfg_attr(not(test), no_std)]

$0

//- /std.rs crate:std deps:core

pub mod fmt {
    pub use core::fmt::Error;
}

//- /zzz.rs crate:core

pub mod fmt {
    pub struct Error;
}
        "# , "core::fmt::Error" , expect ! [[r#"
                Plain  (imports ✔): core::fmt::Error
                Plain  (imports ✖): core::fmt::Error
                ByCrate(imports ✔): core::fmt::Error
                ByCrate(imports ✖): core::fmt::Error
                BySelf (imports ✔): core::fmt::Error
                BySelf (imports ✖): core::fmt::Error
            "#]] ,) ; } # [test] fn prefer_alloc_paths_over_std () { check_found_path (r#"
//- /main.rs crate:main deps:alloc,std
#![no_std]

extern crate alloc;

$0

//- /std.rs crate:std deps:alloc

pub mod sync {
    pub use alloc::sync::Arc;
}

//- /zzz.rs crate:alloc

pub mod sync {
    pub struct Arc;
}
            "# , "alloc::sync::Arc" , expect ! [[r#"
                Plain  (imports ✔): alloc::sync::Arc
                Plain  (imports ✖): alloc::sync::Arc
                ByCrate(imports ✔): alloc::sync::Arc
                ByCrate(imports ✖): alloc::sync::Arc
                BySelf (imports ✔): alloc::sync::Arc
                BySelf (imports ✖): alloc::sync::Arc
            "#]] ,) ; } # [test] fn prefer_shorter_paths_if_not_alloc () { check_found_path (r#"
//- /main.rs crate:main deps:megaalloc,std
$0

//- /std.rs crate:std deps:megaalloc
pub mod sync {
    pub use megaalloc::sync::Arc;
}

//- /zzz.rs crate:megaalloc
pub struct Arc;
            "# , "megaalloc::Arc" , expect ! [[r#"
                Plain  (imports ✔): megaalloc::Arc
                Plain  (imports ✖): megaalloc::Arc
                ByCrate(imports ✔): megaalloc::Arc
                ByCrate(imports ✖): megaalloc::Arc
                BySelf (imports ✔): megaalloc::Arc
                BySelf (imports ✖): megaalloc::Arc
            "#]] ,) ; } # [test] fn builtins_are_in_scope () { let code = r#"
$0

pub mod primitive {
    pub use u8;
}
        "# ; check_found_path (code , "u8" , expect ! [[r#"
                Plain  (imports ✔): u8
                Plain  (imports ✖): u8
                ByCrate(imports ✔): u8
                ByCrate(imports ✖): u8
                BySelf (imports ✔): u8
                BySelf (imports ✖): u8
            "#]] ,) ; check_found_path (code , "u16" , expect ! [[r#"
                Plain  (imports ✔): u16
                Plain  (imports ✖): u16
                ByCrate(imports ✔): u16
                ByCrate(imports ✖): u16
                BySelf (imports ✔): u16
                BySelf (imports ✖): u16
            "#]] ,) ; } # [test] fn inner_items () { check_found_path (r#"
fn main() {
    struct Inner {}
    $0
}
        "# , "Inner" , expect ! [[r#"
                Plain  (imports ✔): Inner
                Plain  (imports ✖): Inner
                ByCrate(imports ✔): Inner
                ByCrate(imports ✖): Inner
                BySelf (imports ✔): Inner
                BySelf (imports ✖): Inner
            "#]] ,) ; } # [test] fn inner_items_from_outer_scope () { check_found_path (r#"
fn main() {
    struct Struct {}
    {
        $0
    }
}
        "# , "Struct" , expect ! [[r#"
                Plain  (imports ✔): Struct
                Plain  (imports ✖): Struct
                ByCrate(imports ✔): Struct
                ByCrate(imports ✖): Struct
                BySelf (imports ✔): Struct
                BySelf (imports ✖): Struct
            "#]] ,) ; } # [test] fn inner_items_from_inner_module () { check_found_path (r#"
fn main() {
    mod module {
        pub struct Struct {}
    }
    {
        $0
    }
}
        "# , "module::Struct" , expect ! [[r#"
                Plain  (imports ✔): module::Struct
                Plain  (imports ✖): module::Struct
                ByCrate(imports ✔): module::Struct
                ByCrate(imports ✖): module::Struct
                BySelf (imports ✔): module::Struct
                BySelf (imports ✖): module::Struct
            "#]] ,) ; } # [test] fn outer_items_with_inner_items_present () { check_found_path (r#"
mod module {
    pub struct CompleteMe;
}

fn main() {
    fn inner() {}
    $0
}
            "# , "module::CompleteMe" , expect ! [[r#"
                Plain  (imports ✔): module::CompleteMe
                Plain  (imports ✖): module::CompleteMe
                ByCrate(imports ✔): crate::module::CompleteMe
                ByCrate(imports ✖): crate::module::CompleteMe
                BySelf (imports ✔): self::module::CompleteMe
                BySelf (imports ✖): self::module::CompleteMe
            "#]] ,) } # [test] fn from_inside_module () { check_found_path (r#"
mod baz {
    pub struct Foo {}
}

mod bar {
    fn bar() {
        $0
    }
}
            "# , "crate::baz::Foo" , expect ! [[r#"
                Plain  (imports ✔): crate::baz::Foo
                Plain  (imports ✖): crate::baz::Foo
                ByCrate(imports ✔): crate::baz::Foo
                ByCrate(imports ✖): crate::baz::Foo
                BySelf (imports ✔): crate::baz::Foo
                BySelf (imports ✖): crate::baz::Foo
            "#]] ,) } # [test] fn from_inside_module2 () { check_found_path (r#"
mod qux {
    pub mod baz {
        pub struct Foo {}
    }

    mod bar {
        fn bar() {
            $0;
        }
    }
}

            "# , "crate::qux::baz::Foo" , expect ! [[r#"
                Plain  (imports ✔): super::baz::Foo
                Plain  (imports ✖): super::baz::Foo
                ByCrate(imports ✔): crate::qux::baz::Foo
                ByCrate(imports ✖): crate::qux::baz::Foo
                BySelf (imports ✔): super::baz::Foo
                BySelf (imports ✖): super::baz::Foo
            "#]] ,) } # [test] fn from_inside_module_with_inner_items () { check_found_path (r#"
mod baz {
    pub struct Foo {}
}

mod bar {
    fn bar() {
        fn inner() {}
        $0
    }
}
            "# , "crate::baz::Foo" , expect ! [[r#"
                Plain  (imports ✔): crate::baz::Foo
                Plain  (imports ✖): crate::baz::Foo
                ByCrate(imports ✔): crate::baz::Foo
                ByCrate(imports ✖): crate::baz::Foo
                BySelf (imports ✔): crate::baz::Foo
                BySelf (imports ✖): crate::baz::Foo
            "#]] ,) } # [test] fn recursive_pub_mod_reexport () { check_found_path (r#"
fn main() {
    let _ = 22_i32.as_name$0();
}

pub mod name {
    pub trait AsName {
        fn as_name(&self) -> String;
    }
    impl AsName for i32 {
        fn as_name(&self) -> String {
            format!("Name: {}", self)
        }
    }
    pub use crate::name;
}
"# , "name::AsName" , expect ! [[r#"
                Plain  (imports ✔): name::AsName
                Plain  (imports ✖): name::AsName
                ByCrate(imports ✔): crate::name::AsName
                ByCrate(imports ✖): crate::name::AsName
                BySelf (imports ✔): self::name::AsName
                BySelf (imports ✖): self::name::AsName
            "#]] ,) ; } # [test] fn extern_crate () { check_found_path (r#"
//- /main.rs crate:main deps:dep
$0
//- /dep.rs crate:dep
"# , "dep" , expect ! [[r#"
                Plain  (imports ✔): dep
                Plain  (imports ✖): dep
                ByCrate(imports ✔): dep
                ByCrate(imports ✖): dep
                BySelf (imports ✔): dep
                BySelf (imports ✖): dep
            "#]] ,) ; check_found_path (r#"
//- /main.rs crate:main deps:dep
fn f() {
    fn inner() {}
    $0
}
//- /dep.rs crate:dep
"# , "dep" , expect ! [[r#"
                Plain  (imports ✔): dep
                Plain  (imports ✖): dep
                ByCrate(imports ✔): dep
                ByCrate(imports ✖): dep
                BySelf (imports ✔): dep
                BySelf (imports ✖): dep
            "#]] ,) ; } # [test] fn prelude_with_inner_items () { check_found_path (r#"
//- /main.rs edition:2018 crate:main deps:std
fn f() {
    fn inner() {}
    $0
}
//- /std.rs crate:std
pub mod prelude {
    pub mod rust_2018 {
        pub enum Option { None }
        pub use Option::*;
    }
}
        "# , "None" , expect ! [[r#"
                Plain  (imports ✔): None
                Plain  (imports ✖): None
                ByCrate(imports ✔): None
                ByCrate(imports ✖): None
                BySelf (imports ✔): None
                BySelf (imports ✖): None
            "#]] ,) ; } # [test] fn different_crate_renamed_through_dep () { check_found_path (r#"
//- /main.rs crate:main deps:intermediate
$0
//- /intermediate.rs crate:intermediate deps:std
pub extern crate std as std_renamed;
//- /std.rs crate:std
pub struct S;
    "# , "intermediate::std_renamed::S" , expect ! [[r#"
                Plain  (imports ✔): intermediate::std_renamed::S
                Plain  (imports ✖): intermediate::std_renamed::S
                ByCrate(imports ✔): intermediate::std_renamed::S
                ByCrate(imports ✖): intermediate::std_renamed::S
                BySelf (imports ✔): intermediate::std_renamed::S
                BySelf (imports ✖): intermediate::std_renamed::S
            "#]] ,) ; } # [test] fn different_crate_doc_hidden () { check_found_path (r#"
//- /main.rs crate:main deps:intermediate
$0
//- /intermediate.rs crate:intermediate deps:std
#[doc(hidden)]
pub extern crate std;
pub extern crate std as longer;
//- /std.rs crate:std
pub struct S;
    "# , "intermediate::longer::S" , expect ! [[r#"
                Plain  (imports ✔): intermediate::longer::S
                Plain  (imports ✖): intermediate::longer::S
                ByCrate(imports ✔): intermediate::longer::S
                ByCrate(imports ✖): intermediate::longer::S
                BySelf (imports ✔): intermediate::longer::S
                BySelf (imports ✖): intermediate::longer::S
            "#]] ,) ; } # [test] fn respect_doc_hidden () { check_found_path (r#"
//- /main.rs crate:main deps:std,lazy_static
$0
//- /lazy_static.rs crate:lazy_static deps:core
#[doc(hidden)]
pub use core::ops::Deref as __Deref;
//- /std.rs crate:std deps:core
pub use core::ops;
//- /core.rs crate:core
pub mod ops {
    pub trait Deref {}
}
    "# , "std::ops::Deref" , expect ! [[r#"
                Plain  (imports ✔): std::ops::Deref
                Plain  (imports ✖): std::ops::Deref
                ByCrate(imports ✔): std::ops::Deref
                ByCrate(imports ✖): std::ops::Deref
                BySelf (imports ✔): std::ops::Deref
                BySelf (imports ✖): std::ops::Deref
            "#]] ,) ; } # [test] fn respect_unstable_modules () { check_found_path_prefer_no_std_allow_unstable (r#"
//- /main.rs crate:main deps:std,core
extern crate std;
$0
//- /longer.rs crate:std deps:core
pub mod error {
    pub use core::error::Error;
}
//- /core.rs crate:core
pub mod error {
    #![unstable(feature = "error_in_core", issue = "103765")]
    pub trait Error {}
}
"# , "std::error::Error" , expect ! [[r#"
                Plain  (imports ✔): std::error::Error
                Plain  (imports ✖): std::error::Error
                ByCrate(imports ✔): std::error::Error
                ByCrate(imports ✖): std::error::Error
                BySelf (imports ✔): std::error::Error
                BySelf (imports ✖): std::error::Error
            "#]] ,) ; } # [test] fn respects_prelude_setting () { let ra_fixture = r#"
//- /main.rs crate:main deps:krate
$0
//- /krate.rs crate:krate
pub mod prelude {
    pub use crate::foo::*;
}

pub mod foo {
    pub struct Foo;
}
"# ; check_found_path (ra_fixture , "krate::foo::Foo" , expect ! [[r#"
                Plain  (imports ✔): krate::foo::Foo
                Plain  (imports ✖): krate::foo::Foo
                ByCrate(imports ✔): krate::foo::Foo
                ByCrate(imports ✖): krate::foo::Foo
                BySelf (imports ✔): krate::foo::Foo
                BySelf (imports ✖): krate::foo::Foo
            "#]] ,) ; check_found_path_prelude (ra_fixture , "krate::prelude::Foo" , expect ! [[r#"
                Plain  (imports ✔): krate::prelude::Foo
                Plain  (imports ✖): krate::prelude::Foo
                ByCrate(imports ✔): krate::prelude::Foo
                ByCrate(imports ✖): krate::prelude::Foo
                BySelf (imports ✔): krate::prelude::Foo
                BySelf (imports ✖): krate::prelude::Foo
            "#]] ,) ; } # [test] fn respects_absolute_setting () { let ra_fixture = r#"
//- /main.rs crate:main deps:krate
$0
//- /krate.rs crate:krate
pub mod foo {
    pub struct Foo;
}
"# ; check_found_path (ra_fixture , "krate::foo::Foo" , expect ! [[r#"
            Plain  (imports ✔): krate::foo::Foo
            Plain  (imports ✖): krate::foo::Foo
            ByCrate(imports ✔): krate::foo::Foo
            ByCrate(imports ✖): krate::foo::Foo
            BySelf (imports ✔): krate::foo::Foo
            BySelf (imports ✖): krate::foo::Foo
        "#]] ,) ; check_found_path_absolute (ra_fixture , "krate::foo::Foo" , expect ! [[r#"
            Plain  (imports ✔): ::krate::foo::Foo
            Plain  (imports ✖): ::krate::foo::Foo
            ByCrate(imports ✔): ::krate::foo::Foo
            ByCrate(imports ✖): ::krate::foo::Foo
            BySelf (imports ✔): ::krate::foo::Foo
            BySelf (imports ✖): ::krate::foo::Foo
        "#]] ,) ; } # [test] fn respect_segment_length () { check_found_path (r#"
//- /main.rs crate:main deps:petgraph
$0
//- /petgraph.rs crate:petgraph
pub mod graph {
    pub use crate::graph_impl::{
        NodeIndex
    };
}

mod graph_impl {
    pub struct NodeIndex<Ix>(Ix);
}

pub mod stable_graph {
    #[doc(no_inline)]
    pub use crate::graph::{NodeIndex};
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::graph::{NodeIndex};
}
"# , "petgraph::graph::NodeIndex" , expect ! [[r#"
                Plain  (imports ✔): petgraph::graph::NodeIndex
                Plain  (imports ✖): petgraph::graph::NodeIndex
                ByCrate(imports ✔): petgraph::graph::NodeIndex
                ByCrate(imports ✖): petgraph::graph::NodeIndex
                BySelf (imports ✔): petgraph::graph::NodeIndex
                BySelf (imports ✖): petgraph::graph::NodeIndex
            "#]] ,) ; } # [test] fn regression_17271 () { check_found_path (r#"
//- /lib.rs crate:main
mod foo;

//- /foo.rs
mod bar;

pub fn b() {$0}
//- /foo/bar.rs
pub fn c() {}
"# , "bar::c" , expect ! [[r#"
                Plain  (imports ✔): bar::c
                Plain  (imports ✖): bar::c
                ByCrate(imports ✔): crate::foo::bar::c
                ByCrate(imports ✖): crate::foo::bar::c
                BySelf (imports ✔): self::bar::c
                BySelf (imports ✖): self::bar::c
            "#]] ,) ; } # [test] fn prefer_long_std_over_short_extern () { check_found_path (r#"
//- /lib.rs crate:main deps:futures_lite,std,core
$0
//- /futures_lite.rs crate:futures_lite deps:std,core
pub use crate::future::Future;
pub mod future {
    pub use core::future::Future;
}
//- /std.rs crate:std deps:core
pub use core::future;
//- /core.rs crate:core
pub mod future {
    pub trait Future {}
}
"# , "core::future::Future" , expect ! [[r#"
                Plain  (imports ✔): std::future::Future
                Plain  (imports ✖): std::future::Future
                ByCrate(imports ✔): std::future::Future
                ByCrate(imports ✖): std::future::Future
                BySelf (imports ✔): std::future::Future
                BySelf (imports ✖): std::future::Future
            "#]] ,) ; } }
};
}
