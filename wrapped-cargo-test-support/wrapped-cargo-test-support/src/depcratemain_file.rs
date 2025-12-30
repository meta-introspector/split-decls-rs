// Generated macro for main_file (function)
macro_rules! Depcratemain_file {
() => {
// Module: crate
// Provides: {"main_file"}
// Dependencies: {}
# [doc = " Generate a `main.rs` printing the specified text"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use cargo_test_support::main_file;"] # [doc = " # mod dep {"] # [doc = " #     fn bar() -> &'static str {"] # [doc = " #         \"world\""] # [doc = " #     }"] # [doc = " # }"] # [doc = " main_file("] # [doc = "     r#\"\"hello {}\", dep::bar()\"#,"] # [doc = "     &[]"] # [doc = " );"] # [doc = " ```"] pub fn main_file (println : & str , externed_deps : & [& str]) -> String { let mut buf = String :: new () ; for dep in externed_deps . iter () { buf . push_str (& format ! ("extern crate {};\n" , dep)) ; } buf . push_str ("fn main() { println!(") ; buf . push_str (println) ; buf . push_str ("); }\n") ; buf }
};
}
