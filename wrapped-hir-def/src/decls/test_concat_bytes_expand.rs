macro_rules! test_concat_bytes_expand {
    () => {
        # [test] fn test_concat_bytes_expand () { check (r##"
#[rustc_builtin_macro]
macro_rules! concat_bytes {}

fn main() { concat_bytes!(b'A', b"BC\"", [68, b'E', 70], br#"G""#,b'\0'); }
"## , expect ! [[r#"
#[rustc_builtin_macro]
macro_rules! concat_bytes {}

fn main() { b"ABC\"DEFG\"\x00"; }
"#]] ,) ; }
    };
}

test_concat_bytes_expand!();