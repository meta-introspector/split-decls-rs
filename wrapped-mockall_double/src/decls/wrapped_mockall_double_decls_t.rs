use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod t {
    use super::*;
    mod double {
        use super::*;
        use std::str::FromStr;
        fn cmp(attrs: &str, code: &str, expected: &str) {
            let attrs_ts = TokenStream::from_str(attrs).unwrap();
            let code_ts = TokenStream::from_str(code).unwrap();
            let output = do_double(attrs_ts, code_ts);
            let output = output.to_string();
            let expected = TokenStream::from_str(expected).unwrap().to_string();
            assert_eq!(output, expected);
        }
        #[test]
        #[should_panic(expected = "Cannot double glob")]
        fn glob() {
            let code = "use foo::*;";
            cmp("", code, "");
        }
        #[test]
        fn group() {
            let code = "
            use foo::bar::{
                Baz,
                Bean
            };
        ";
            let expected = "
            #[cfg(not(test))]
            use foo::bar::{
                Baz,
                Bean
            };
            #[cfg(test)]
            use foo::bar::{
                MockBaz as Baz,
                MockBean as Bean
            };
        ";
            cmp("", code, expected);
        }
        #[test]
        fn module() {
            let code = "use foo::bar;";
            let expected = "
            #[cfg(not(test))]
            use foo::bar;
            #[cfg(test)]
            use foo::mock_bar as bar;
        ";
            cmp("", code, expected);
        }
        #[test]
        #[should_panic(expected = "Cannot double types in the current module")]
        fn name() {
            let code = "use Foo;";
            cmp("", code, "");
        }
        #[test]
        fn path() {
            let code = "use foo::bar::Baz;";
            let expected = "
            #[cfg(not(test))]
            use foo::bar::Baz;
            #[cfg(test)]
            use foo::bar::MockBaz as Baz;
        ";
            cmp("", code, expected);
        }
        #[test]
        fn pub_use() {
            let code = "pub use foo::bar;";
            let expected = "
            #[cfg(not(test))]
            pub use foo::bar;
            #[cfg(test)]
            pub use foo::mock_bar as bar;
        ";
            cmp("", code, expected);
        }
        #[test]
        fn rename() {
            let code = "use Foo as Bar;";
            let expected = "
            #[cfg(not(test))]
            use Foo as Bar;
            #[cfg(test)]
            use MockFoo as Bar;
        ";
            cmp("", code, expected);
        }
        #[test]
        fn type_() {
            let code = "type Foo = bar::Baz;";
            let expected = "
            #[cfg(not(test))]
            type Foo = bar::Baz;
            #[cfg(test)]
            type Foo = bar::MockBaz;
        ";
            cmp("", code, expected);
        }
        #[test]
        #[should_panic(expected = "Only use statements and type aliases")]
        fn undoubleable() {
            let code = "struct Foo{}";
            cmp("", code, "");
        }
    }
}
