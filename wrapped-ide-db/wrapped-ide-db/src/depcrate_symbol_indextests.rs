// Generated macro for tests (module)
macro_rules! Depcrate_symbol_indextests {
() => {
// Module: crate::symbol_index
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use expect_test :: expect_file ; use salsa :: Setter ; use test_fixture :: { WORKSPACE , WithFixture } ; use super :: * ; # [test] fn test_symbol_index_collection () { let (db , _) = RootDatabase :: with_many_files (r#"
//- /main.rs

macro_rules! macro_rules_macro {
    () => {}
};

macro_rules! define_struct {
    () => {
        struct StructFromMacro;
    }
};

define_struct!();

macro Macro { }

struct Struct;
enum Enum {
    A, B
}
union Union {}

impl Struct {
    fn impl_fn() {}
}

struct StructT<T>;

impl <T> StructT<T> {
    fn generic_impl_fn() {}
}

trait Trait {
    fn trait_fn(&self);
}

fn main() {
    struct StructInFn;
}

const CONST: u32 = 1;
static STATIC: &'static str = "2";
type Alias = Struct;

mod a_mod {
    struct StructInModA;
}

const _: () = {
    struct StructInUnnamedConst;

    ()
};

const CONST_WITH_INNER: () = {
    struct StructInNamedConst;

    ()
};

mod b_mod;


use define_struct as really_define_struct;
use Macro as ItemLikeMacro;
use Macro as Trait; // overlay namespaces
//- /b_mod.rs
struct StructInModB;
pub(self) use super::Macro as SuperItemLikeMacro;
pub(self) use crate::b_mod::StructInModB as ThisStruct;
pub(self) use crate::Trait as IsThisJustATrait;
"# ,) ; let symbols : Vec < _ > = Crate :: from (db . test_crate ()) . modules (& db) . into_iter () . map (| module_id | { let mut symbols = SymbolCollector :: new_module (& db , module_id , false) ; symbols . sort_by_key (| it | it . name . as_str () . to_owned ()) ; (module_id , symbols) }) . collect () ; expect_file ! ["./test_data/test_symbol_index_collection.txt"] . assert_debug_eq (& symbols) ; } # [test] fn test_doc_alias () { let (db , _) = RootDatabase :: with_single_file (r#"
#[doc(alias="s1")]
#[doc(alias="s2")]
#[doc(alias("mul1","mul2"))]
struct Struct;

#[doc(alias="s1")]
struct Duplicate;
        "# ,) ; let symbols : Vec < _ > = Crate :: from (db . test_crate ()) . modules (& db) . into_iter () . map (| module_id | { let mut symbols = SymbolCollector :: new_module (& db , module_id , false) ; symbols . sort_by_key (| it | it . name . as_str () . to_owned ()) ; (module_id , symbols) }) . collect () ; expect_file ! ["./test_data/test_doc_alias.txt"] . assert_debug_eq (& symbols) ; } # [test] fn test_exclude_imports () { let (mut db , _) = RootDatabase :: with_many_files (r#"
//- /lib.rs
mod foo;
pub use foo::Foo;

//- /foo.rs
pub struct Foo;
"# ,) ; let mut local_roots = FxHashSet :: default () ; local_roots . insert (WORKSPACE) ; LocalRoots :: get (& db) . set_roots (& mut db) . to (local_roots) ; let mut query = Query :: new ("Foo" . to_owned ()) ; let mut symbols = world_symbols (& db , query . clone ()) ; symbols . sort_by_key (| x | x . is_import) ; expect_file ! ["./test_data/test_symbols_with_imports.txt"] . assert_debug_eq (& symbols) ; query . exclude_imports () ; let symbols = world_symbols (& db , query) ; expect_file ! ["./test_data/test_symbols_exclude_imports.txt"] . assert_debug_eq (& symbols) ; } }
};
}
