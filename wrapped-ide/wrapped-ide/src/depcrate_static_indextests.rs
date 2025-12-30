// Generated macro for tests (module)
macro_rules! Depcrate_static_indextests {
() => {
// Module: crate::static_index
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { StaticIndex , fixture } ; use ide_db :: { FileRange , FxHashMap , FxHashSet , base_db :: VfsPath } ; use syntax :: TextSize ; use super :: VendoredLibrariesConfig ; fn check_all_ranges (# [rust_analyzer :: rust_fixture] ra_fixture : & str , vendored_libs_config : VendoredLibrariesConfig < '_ > ,) { let (analysis , ranges) = fixture :: annotations_without_marker (ra_fixture) ; let s = StaticIndex :: compute (& analysis , vendored_libs_config) ; let mut range_set : FxHashSet < _ > = ranges . iter () . map (| it | it . 0) . collect () ; for f in s . files { for (range , _) in f . tokens { if range . start () == TextSize :: from (0) { continue ; } let it = FileRange { file_id : f . file_id , range } ; if ! range_set . contains (& it) { panic ! ("additional range {it:?}") ; } range_set . remove (& it) ; } } if ! range_set . is_empty () { panic ! ("unfound ranges {range_set:?}") ; } } # [track_caller] fn check_definitions (# [rust_analyzer :: rust_fixture] ra_fixture : & str , vendored_libs_config : VendoredLibrariesConfig < '_ > ,) { let (analysis , ranges) = fixture :: annotations_without_marker (ra_fixture) ; let s = StaticIndex :: compute (& analysis , vendored_libs_config) ; let mut range_set : FxHashSet < _ > = ranges . iter () . map (| it | it . 0) . collect () ; for (_ , t) in s . tokens . iter () { if let Some (t) = t . definition { if t . range . start () == TextSize :: from (0) { continue ; } if ! range_set . contains (& t) { panic ! ("additional definition {t:?}") ; } range_set . remove (& t) ; } } if ! range_set . is_empty () { panic ! ("unfound definitions {range_set:?}") ; } } # [track_caller] fn check_references (# [rust_analyzer :: rust_fixture] ra_fixture : & str , vendored_libs_config : VendoredLibrariesConfig < '_ > ,) { let (analysis , ranges) = fixture :: annotations_without_marker (ra_fixture) ; let s = StaticIndex :: compute (& analysis , vendored_libs_config) ; let mut range_set : FxHashMap < _ , i32 > = ranges . iter () . map (| it | (it . 0 , 0)) . collect () ; for (_ , t) in s . tokens . iter () { for r in & t . references { if r . is_definition { continue ; } if r . range . range . start () == TextSize :: from (0) { continue ; } match range_set . entry (r . range) { std :: collections :: hash_map :: Entry :: Occupied (mut entry) => { let count = entry . get_mut () ; * count += 1 ; } std :: collections :: hash_map :: Entry :: Vacant (_) => { panic ! ("additional reference {r:?}") ; } } } } for (range , count) in range_set . iter () { if * count == 0 { panic ! ("unfound reference {range:?}") ; } } } # [test] fn field_initialization () { check_references (r#"
struct Point {
    x: f64,
     //^^^
    y: f64,
     //^^^
}
    fn foo() {
        let x = 5.;
        let y = 10.;
        let mut p = Point { x, y };
                  //^^^^^   ^  ^
        p.x = 9.;
      //^ ^
        p.y = 10.;
      //^ ^
    }
"# , VendoredLibrariesConfig :: Included { workspace_root : & VfsPath :: new_virtual_path ("/workspace" . to_owned ()) , } ,) ; } # [test] fn struct_and_enum () { check_all_ranges (r#"
struct Foo;
     //^^^
enum E { X(Foo) }
   //^   ^ ^^^
"# , VendoredLibrariesConfig :: Included { workspace_root : & VfsPath :: new_virtual_path ("/workspace" . to_owned ()) , } ,) ; check_definitions (r#"
struct Foo;
     //^^^
enum E { X(Foo) }
   //^   ^
"# , VendoredLibrariesConfig :: Included { workspace_root : & VfsPath :: new_virtual_path ("/workspace" . to_owned ()) , } ,) ; check_references (r#"
struct Foo;
enum E { X(Foo) }
   //      ^^^
"# , VendoredLibrariesConfig :: Included { workspace_root : & VfsPath :: new_virtual_path ("/workspace" . to_owned ()) , } ,) ; } # [test] fn multi_crate () { check_definitions (r#"
//- /workspace/main.rs crate:main deps:foo


use foo::func;

fn main() {
 //^^^^
    func();
}
//- /workspace/foo/lib.rs crate:foo

pub func() {

}
"# , VendoredLibrariesConfig :: Included { workspace_root : & VfsPath :: new_virtual_path ("/workspace" . to_owned ()) , } ,) ; } # [test] fn vendored_crate () { check_all_ranges (r#"
//- /workspace/main.rs crate:main deps:external,vendored
struct Main(i32);
     //^^^^ ^^^

//- /external/lib.rs new_source_root:library crate:external@0.1.0,https://a.b/foo.git library
struct ExternalLibrary(i32);

//- /workspace/vendored/lib.rs new_source_root:library crate:vendored@0.1.0,https://a.b/bar.git library
struct VendoredLibrary(i32);
     //^^^^^^^^^^^^^^^ ^^^
"# , VendoredLibrariesConfig :: Included { workspace_root : & VfsPath :: new_virtual_path ("/workspace" . to_owned ()) , } ,) ; } # [test] fn vendored_crate_excluded () { check_all_ranges (r#"
//- /workspace/main.rs crate:main deps:external,vendored
struct Main(i32);
     //^^^^ ^^^

//- /external/lib.rs new_source_root:library crate:external@0.1.0,https://a.b/foo.git library
struct ExternalLibrary(i32);

//- /workspace/vendored/lib.rs new_source_root:library crate:vendored@0.1.0,https://a.b/bar.git library
struct VendoredLibrary(i32);
"# , VendoredLibrariesConfig :: Excluded ,) } # [test] fn derives () { check_all_ranges (r#"
//- minicore:derive
#[rustc_builtin_macro]
//^^^^^^^^^^^^^^^^^^^
pub macro Copy {}
        //^^^^
#[derive(Copy)]
//^^^^^^ ^^^^
struct Hello(i32);
     //^^^^^ ^^^
"# , VendoredLibrariesConfig :: Included { workspace_root : & VfsPath :: new_virtual_path ("/workspace" . to_owned ()) , } ,) ; } }
};
}
