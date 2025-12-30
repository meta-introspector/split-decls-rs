// Generated macro for try_compile_and_wrap (function)
macro_rules! Depcratetry_compile_and_wrap {
() => {
// Module: crate
// Provides: {"try_compile_and_wrap"}
// Dependencies: {}
fn try_compile_and_wrap (code : & str) -> Result < String , Box < dyn std :: error :: Error > > { let test_code = format ! (r#"
fn main() {{
    {}
}}
"# , code) ; fs :: write ("/tmp/test_snippet.rs" , & test_code) ? ; let output = Command :: new ("rustc") . args (& ["--crate-type" , "bin" , "/tmp/test_snippet.rs" , "-o" , "/tmp/test_snippet"]) . output () ? ; if output . status . success () { Ok (format ! (r#"
prelude! {{
    // Auto-detected: no imports needed
}}
mkdecl! {{
    {}
}}
"# , code)) } else { let stderr = String :: from_utf8_lossy (& output . stderr) ; let imports = if stderr . contains ("HashMap") { "use std::collections::HashMap;" } else if stderr . contains ("Vec") { "use std::vec::Vec;" } else { "use std::*;" } ; Ok (format ! (r#"
prelude! {{
    {}
}}
mkdecl! {{
    {}
}}
"# , imports , code)) } }
};
}
