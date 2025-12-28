macro_rules! use_declaration_with_braces {
    () => {
        # [test] fn use_declaration_with_braces () { cov_mark :: check ! (use_declaration_with_braces) ; assert_ssr_transform ("foo::bar ==>> foo2::bar2" , r#"
        mod foo { pub(crate) fn bar() {} pub(crate) fn baz() {} }
        mod foo2 { pub(crate) fn bar2() {} }
        use foo::{baz, bar};
        fn main() { bar() }
        "# , expect ! [["
        mod foo { pub(crate) fn bar() {} pub(crate) fn baz() {} }
        mod foo2 { pub(crate) fn bar2() {} }
        use foo::{baz, bar};
        fn main() { foo2::bar2() }
        "]] ,) }
    };
}

use_declaration_with_braces!()