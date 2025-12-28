macro_rules! language_quine_impl {
    () => {
        # [decl (fn , name = "language_quine_impl" , vis = "pub" , hash = "a2f1aef2")] pub fn language_quine_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let language = input_str . value () ; quote ! { { println ! ("cargo:warning=🔄 Generating quine for language: {}" , # language) ; let quine_macro = match # language { "rust" => r###"
macro_rules! rust_quine {
    () => {
        r#"fn main(){print!("fn main(){{print!(\"{}\");}}","fn main(){print!(\"fn main(){{print!(\\\"fn main(){{print!(\\\\\\\"{}\\\\\\\");}}\\\")}}\\\");}}\")}}");"#
    };
}
                "### , "c" => r###"
macro_rules! c_quine {
    () => {
        r#"#include<stdio.h>
int main(){char*s="#include<stdio.h>%cint main(){char*s=%c%s%c;printf(s,10,34,s,34,10);return 0;}%c";printf(s,10,34,s,34,10);return 0;}"#
    };
}
                "### , "python" => r###"
macro_rules! python_quine {
    () => {
        r#"s='s=%r;print(s%%s)';print(s%s)"#
    };
}
                "### , "lean4" => r###"
macro_rules! lean4_quine {
    () => {
        r#"#eval s!"theorem quine : String := sorry""#
    };
}
                "### , _ => r###"
macro_rules! unknown_quine {
    () => {
        r#"// Quine not implemented"#
    };
}
                "### } ; quine_macro . to_string () } } . into () }
    };
}

language_quine_impl!()