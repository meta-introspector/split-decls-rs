// Generated macro for retain_files_callback (function)
macro_rules! Depcrate_testretain_files_callback {
() => {
// Module: crate::test
// Provides: {"retain_files_callback"}
// Dependencies: {}
fn retain_files_callback < 'a > (file_path : & 'a str , test_type : & 'a str ,) -> impl Fn (& Path) -> Result < bool , String > + 'a { move | rust_path | { let files = std :: fs :: read_to_string (file_path) . unwrap_or_default () ; let first_file_name = files . lines () . next () . unwrap_or ("") ; if first_file_name . ends_with ('/') { run_command (& [& "find" , & format ! ("tests/{test_type}") , & "-mindepth" , & "1" , & "-type" , & "d" , & "-exec" , & "rm" , & "-rf" , & "{}" , & "+" ,] , Some (rust_path) ,) ? ; } else { run_command (& [& "find" , & format ! ("tests/{test_type}") , & "-type" , & "f" , & "-name" , & "*.rs" , & "-not" , & "-path" , & "*/auxiliary/*" , & "-delete" ,] , Some (rust_path) ,) ? ; } if let Ok (files) = std :: fs :: read_to_string (file_path) { for file in files . split ('\n') . map (| line | line . trim ()) . filter (| line | ! line . is_empty ()) { run_command (& [& "git" , & "checkout" , & "--" , & file] , Some (rust_path)) ? ; } } else { println ! ("Failed to read `{file_path}`, not putting back failing {test_type} tests") ; } Ok (true) } }
};
}
