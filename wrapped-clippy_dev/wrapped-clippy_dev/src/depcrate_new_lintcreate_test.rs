// Generated macro for create_test (function)
macro_rules! Depcrate_new_lintcreate_test {
() => {
// Module: crate::new_lint
// Provides: {"create_test"}
// Dependencies: {}
fn create_test (lint : & LintData < '_ > , msrv : bool) -> io :: Result < () > { fn create_project_layout < P : Into < PathBuf > > (lint_name : & str , location : P , case : & str , hint : & str , msrv : bool ,) -> io :: Result < () > { let mut path = location . into () . join (case) ; fs :: create_dir (& path) ? ; write_file (path . join ("Cargo.toml") , get_manifest_contents (lint_name , hint)) ? ; path . push ("src") ; fs :: create_dir (& path) ? ; write_file (path . join ("main.rs") , get_test_file_contents (lint_name , msrv)) ? ; Ok (()) } if lint . category == "cargo" { let test_dir = format ! ("tests/ui-cargo/{}" , lint . name) ; fs :: create_dir (& test_dir) ? ; create_project_layout (lint . name , & test_dir , "fail" , "Content that triggers the lint goes here" , msrv ,) ? ; create_project_layout (lint . name , & test_dir , "pass" , "This file should not trigger the lint" , false ,) ? ; println ! ("Generated test directories: `{test_dir}/pass`, `{test_dir}/fail`") ; } else { let test_path = format ! ("tests/ui/{}.rs" , lint . name) ; let test_contents = get_test_file_contents (lint . name , msrv) ; write_file (& test_path , test_contents) ? ; println ! ("Generated test file: `{test_path}`") ; } Ok (()) }
};
}
