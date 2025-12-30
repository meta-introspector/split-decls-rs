// Generated macro for tests (module)
macro_rules! Depcrate_goto_declarationtests {
() => {
// Module: crate::goto_declaration
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use ide_db :: { FileRange , MiniCore } ; use itertools :: Itertools ; use crate :: { GotoDefinitionConfig , fixture } ; const TEST_CONFIG : GotoDefinitionConfig < '_ > = GotoDefinitionConfig { minicore : MiniCore :: default () } ; fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str) { let (analysis , position , expected) = fixture :: annotations (ra_fixture) ; let navs = analysis . goto_declaration (position , & TEST_CONFIG) . unwrap () . expect ("no declaration or definition found") . info ; if navs . is_empty () { panic ! ("unresolved reference") } let cmp = | & FileRange { file_id , range } : & _ | (file_id , range . start ()) ; let navs = navs . into_iter () . map (| nav | FileRange { file_id : nav . file_id , range : nav . focus_or_full_range () }) . sorted_by_key (cmp) . collect :: < Vec < _ > > () ; let expected = expected . into_iter () . map (| (FileRange { file_id , range } , _) | FileRange { file_id , range }) . sorted_by_key (cmp) . collect :: < Vec < _ > > () ; assert_eq ! (expected , navs) ; } # [test] fn goto_decl_module_outline () { check (r#"
//- /main.rs
mod foo;
 // ^^^
//- /foo.rs
use self$0;
"# ,) } # [test] fn goto_decl_module_inline () { check (r#"
mod foo {
 // ^^^
    use self$0;
}
"# ,) } # [test] fn goto_decl_goto_def_fallback () { check (r#"
struct Foo;
    // ^^^
impl Foo$0 {}
"# ,) ; } # [test] fn goto_decl_assoc_item_no_impl_item () { check (r#"
trait Trait {
    const C: () = ();
       // ^
}
impl Trait for () {}

fn main() {
    <()>::C$0;
}
"# ,) ; } # [test] fn goto_decl_assoc_item () { check (r#"
trait Trait {
    const C: () = ();
       // ^
}
impl Trait for () {
    const C: () = ();
}

fn main() {
    <()>::C$0;
}
"# ,) ; check (r#"
trait Trait {
    const C: () = ();
       // ^
}
impl Trait for () {
    const C$0: () = ();
}
"# ,) ; } # [test] fn goto_decl_field_pat_shorthand () { check (r#"
struct Foo { field: u32 }
           //^^^^^
fn main() {
    let Foo { field$0 };
}
"# ,) ; } # [test] fn goto_decl_constructor_shorthand () { check (r#"
struct Foo { field: u32 }
           //^^^^^
fn main() {
    let field = 0;
    Foo { field$0 };
}
"# ,) ; } # [test] fn goto_decl_for_extern_crate () { check (r#"
//- /main.rs crate:main deps:std
extern crate std$0;
         /// ^^^
//- /std/lib.rs crate:std
// empty
"# ,) } # [test] fn goto_decl_for_renamed_extern_crate () { check (r#"
//- /main.rs crate:main deps:std
extern crate std as abc$0;
                /// ^^^
//- /std/lib.rs crate:std
// empty
"# ,) } }
};
}
