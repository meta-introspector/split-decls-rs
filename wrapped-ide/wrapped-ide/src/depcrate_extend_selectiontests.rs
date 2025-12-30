// Generated macro for tests (module)
macro_rules! Depcrate_extend_selectiontests {
() => {
// Module: crate::extend_selection
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: fixture ; use super :: * ; fn do_check (before : & str , afters : & [& str]) { let (analysis , position) = fixture :: position (before) ; let before = analysis . file_text (position . file_id) . unwrap () ; let range = TextRange :: empty (position . offset) ; let mut frange = FileRange { file_id : position . file_id , range } ; for & after in afters { frange . range = analysis . extend_selection (frange) . unwrap () ; let actual = & before [frange . range] ; assert_eq ! (after , actual) ; } } # [test] fn test_extend_selection_arith () { do_check (r#"fn foo() { $01 + 1 }"# , & ["1" , "1 + 1" , "{ 1 + 1 }"]) ; } # [test] fn test_extend_selection_list () { do_check (r#"fn foo($0x: i32) {}"# , & ["x" , "x: i32"]) ; do_check (r#"fn foo($0x: i32, y: i32) {}"# , & ["x" , "x: i32" , "x: i32, "]) ; do_check (r#"fn foo($0x: i32,y: i32) {}"# , & ["x" , "x: i32" , "x: i32," , "(x: i32,y: i32)"]) ; do_check (r#"fn foo(x: i32, $0y: i32) {}"# , & ["y" , "y: i32" , ", y: i32"]) ; do_check (r#"fn foo(x: i32, $0y: i32, ) {}"# , & ["y" , "y: i32" , "y: i32, "]) ; do_check (r#"fn foo(x: i32,$0y: i32) {}"# , & ["y" , "y: i32" , ",y: i32"]) ; do_check (r#"const FOO: [usize; 2] = [ 22$0 , 33];"# , & ["22" , "22 , "]) ; do_check (r#"const FOO: [usize; 2] = [ 22 , 33$0];"# , & ["33" , ", 33"]) ; do_check (r#"const FOO: [usize; 2] = [ 22 , 33$0 ,];"# , & ["33" , "33 ," , "[ 22 , 33 ,]"]) ; do_check (r#"fn main() { (1, 2$0) }"# , & ["2" , ", 2" , "(1, 2)"]) ; do_check (r#"
const FOO: [usize; 2] = [
    22,
    $033,
]"# , & ["33" , "33,"] ,) ; do_check (r#"
const FOO: [usize; 2] = [
    22
    , 33$0,
]"# , & ["33" , "33,"] ,) ; } # [test] fn test_extend_selection_start_of_the_line () { do_check (r#"
impl S {
$0    fn foo() {

    }
}"# , & ["    fn foo() {\n\n    }\n"] ,) ; } # [test] fn test_extend_selection_doc_comments () { do_check (r#"
struct A;

/// bla
/// bla
struct B {
    $0
}
            "# , & ["\n    \n" , "{\n    \n}" , "/// bla\n/// bla\nstruct B {\n    \n}"] ,) } # [test] fn test_extend_selection_comments () { do_check (r#"
fn bar(){}

// fn foo() {
// 1 + $01
// }

// fn foo(){}
    "# , & ["1" , "// 1 + 1" , "// fn foo() {\n// 1 + 1\n// }"] ,) ; do_check (r#"
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Direction {
//  $0   Next,
//     Prev
// }
"# , & ["//     Next," , "// #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n// pub enum Direction {\n//     Next,\n//     Prev\n// }" ,] ,) ; do_check (r#"
/*
foo
_bar1$0*/
"# , & ["_bar1" , "/*\nfoo\n_bar1*/"] ,) ; do_check (r#"//!$0foo_2 bar"# , & ["foo_2" , "//!foo_2 bar"]) ; do_check (r#"/$0/foo bar"# , & ["//foo bar"]) ; } # [test] fn test_extend_selection_prefer_idents () { do_check (r#"
fn main() { foo$0+bar;}
"# , & ["foo" , "foo+bar"] ,) ; do_check (r#"
fn main() { foo+$0bar;}
"# , & ["bar" , "foo+bar"] ,) ; } # [test] fn test_extend_selection_prefer_lifetimes () { do_check (r#"fn foo<$0'a>() {}"# , & ["'a" , "<'a>"]) ; do_check (r#"fn foo<'a$0>() {}"# , & ["'a" , "<'a>"]) ; } # [test] fn test_extend_selection_select_first_word () { do_check (r#"// foo bar b$0az quxx"# , & ["baz" , "// foo bar baz quxx"]) ; do_check (r#"
impl S {
fn foo() {
// hel$0lo world
}
}
"# , & ["hello" , "// hello world"] ,) ; } # [test] fn test_extend_selection_string () { do_check (r#"
fn bar(){}

" fn f$0oo() {"
"# , & ["foo" , "\" fn foo() {\""] ,) ; } # [test] fn test_extend_trait_bounds_list_in_where_clause () { do_check (r#"
fn foo<R>()
    where
        R: req::Request + 'static,
        R::Params: DeserializeOwned$0 + panic::UnwindSafe + 'static,
        R::Result: Serialize + 'static,
"# , & ["DeserializeOwned" , "DeserializeOwned + " , "DeserializeOwned + panic::UnwindSafe + 'static" , "R::Params: DeserializeOwned + panic::UnwindSafe + 'static" , "R::Params: DeserializeOwned + panic::UnwindSafe + 'static," ,] ,) ; do_check (r#"fn foo<T>() where T: $0Copy"# , & ["Copy"]) ; do_check (r#"fn foo<T>() where T: $0Copy + Display"# , & ["Copy" , "Copy + "]) ; do_check (r#"fn foo<T>() where T: $0Copy +Display"# , & ["Copy" , "Copy +"]) ; do_check (r#"fn foo<T>() where T: $0Copy+Display"# , & ["Copy" , "Copy+"]) ; do_check (r#"fn foo<T>() where T: Copy + $0Display"# , & ["Display" , "+ Display"]) ; do_check (r#"fn foo<T>() where T: Copy + $0Display + Sync"# , & ["Display" , "Display + "]) ; do_check (r#"fn foo<T>() where T: Copy +$0Display"# , & ["Display" , "+Display"]) ; } # [test] fn test_extend_trait_bounds_list_inline () { do_check (r#"fn foo<T: $0Copy>() {}"# , & ["Copy"]) ; do_check (r#"fn foo<T: $0Copy + Display>() {}"# , & ["Copy" , "Copy + "]) ; do_check (r#"fn foo<T: $0Copy +Display>() {}"# , & ["Copy" , "Copy +"]) ; do_check (r#"fn foo<T: $0Copy+Display>() {}"# , & ["Copy" , "Copy+"]) ; do_check (r#"fn foo<T: Copy + $0Display>() {}"# , & ["Display" , "+ Display"]) ; do_check (r#"fn foo<T: Copy + $0Display + Sync>() {}"# , & ["Display" , "Display + "]) ; do_check (r#"fn foo<T: Copy +$0Display>() {}"# , & ["Display" , "+Display"]) ; do_check (r#"fn foo<T: Copy$0 + Display, U: Copy>() {}"# , & ["Copy" , "Copy + " , "Copy + Display" , "T: Copy + Display" , "T: Copy + Display, " , "<T: Copy + Display, U: Copy>" ,] ,) ; } # [test] fn test_extend_selection_on_tuple_in_type () { do_check (r#"fn main() { let _: (krate, $0_crate_def_map, module_id) = (); }"# , & ["_crate_def_map" , "_crate_def_map, " , "(krate, _crate_def_map, module_id)"] ,) ; do_check (r#"fn main() { let _: (krate,$0_crate_def_map,module_id) = (); }"# , & ["_crate_def_map" , "_crate_def_map," , "(krate,_crate_def_map,module_id)"] ,) ; do_check (r#"
fn main() { let _: (
    krate,
    _crate$0_def_map,
    module_id
) = (); }"# , & ["_crate_def_map" , "_crate_def_map," , "(\n    krate,\n    _crate_def_map,\n    module_id\n)" ,] ,) ; } # [test] fn test_extend_selection_on_tuple_in_rvalue () { do_check (r#"fn main() { let var = (krate, _crate_def_map$0, module_id); }"# , & ["_crate_def_map" , "_crate_def_map, " , "(krate, _crate_def_map, module_id)"] ,) ; do_check (r#"fn main() { let var = (krate,_crate$0_def_map,module_id); }"# , & ["_crate_def_map" , "_crate_def_map," , "(krate,_crate_def_map,module_id)"] ,) ; do_check (r#"
fn main() { let var = (
    krate,
    _crate_def_map$0,
    module_id
); }"# , & ["_crate_def_map" , "_crate_def_map," , "(\n    krate,\n    _crate_def_map,\n    module_id\n)" ,] ,) ; } # [test] fn test_extend_selection_on_tuple_pat () { do_check (r#"fn main() { let (krate, _crate_def_map$0, module_id) = var; }"# , & ["_crate_def_map" , "_crate_def_map, " , "(krate, _crate_def_map, module_id)"] ,) ; do_check (r#"fn main() { let (krate,_crate$0_def_map,module_id) = var; }"# , & ["_crate_def_map" , "_crate_def_map," , "(krate,_crate_def_map,module_id)"] ,) ; do_check (r#"
fn main() { let (
    krate,
    _crate_def_map$0,
    module_id
) = var; }"# , & ["_crate_def_map" , "_crate_def_map," , "(\n    krate,\n    _crate_def_map,\n    module_id\n)" ,] ,) ; } # [test] fn extend_selection_inside_macros () { do_check (r#"macro_rules! foo { ($item:item) => {$item} }
                foo!{fn hello(na$0me:usize){}}"# , & ["name" , "name:usize" , "(name:usize)" , "fn hello(name:usize){}" , "{fn hello(name:usize){}}" , "foo!{fn hello(name:usize){}}" ,] ,) ; } # [test] fn extend_selection_inside_recur_macros () { do_check (r#" macro_rules! foo2 { ($item:item) => {$item} }
                macro_rules! foo { ($item:item) => {foo2!($item);} }
                foo!{fn hello(na$0me:usize){}}"# , & ["name" , "name:usize" , "(name:usize)" , "fn hello(name:usize){}" , "{fn hello(name:usize){}}" , "foo!{fn hello(name:usize){}}" ,] ,) ; } # [test] fn extend_selection_inside_str_with_wide_char () { do_check (r#"fn main() { let x = "═$0═══════"; }"# , & [r#""════════""# , r#"let x = "════════";"# , r#"{ let x = "════════"; }"# , r#"fn main() { let x = "════════"; }"# ,] ,) ; } }
};
}
