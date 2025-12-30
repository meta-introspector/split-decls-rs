// Generated macro for tests (module)
macro_rules! Depcrate_runnablestests {
() => {
// Module: crate::runnables
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use expect_test :: { Expect , expect } ; use crate :: fixture ; fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let (analysis , position) = fixture :: position (ra_fixture) ; let result = analysis . runnables (position . file_id) . unwrap () . into_iter () . map (| runnable | { let mut a = format ! ("({:?}, {:?}" , runnable . kind . disc () , runnable . nav) ; if runnable . use_name_in_title { a . push_str (", true") ; } if let Some (cfg) = runnable . cfg { a . push_str (& format ! (", {cfg:?}")) ; } a . push (')') ; a }) . collect :: < Vec < _ > > () ; expect . assert_debug_eq (& result) ; } fn check_tests (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let (analysis , position) = fixture :: position (ra_fixture) ; let tests = analysis . related_tests (position , None) . unwrap () ; let navigation_targets = tests . into_iter () . map (| runnable | runnable . nav) . collect :: < Vec < _ > > () ; expect . assert_debug_eq (& navigation_targets) ; } # [test] fn test_runnables () { check (r#"
//- /lib.rs
$0
fn main() {}

#[export_name = "main"]
fn __cortex_m_rt_main_trampoline() {}

#[unsafe(export_name = "main")]
fn __cortex_m_rt_main_trampoline_unsafe() {}

#[test]
fn test_foo() {}

#[::core::prelude::v1::test]
fn test_full_path() {}

#[test]
#[ignore]
fn test_foo() {}

#[bench]
fn bench() {}

mod not_a_root {
    fn main() {}
}
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 0..331, name: \"_\", kind: Module })",
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 1..13, focus_range: 4..8, name: \"main\", kind: Function })",
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 15..76, focus_range: 42..71, name: \"__cortex_m_rt_main_trampoline\", kind: Function })",
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 78..154, focus_range: 113..149, name: \"__cortex_m_rt_main_trampoline_unsafe\", kind: Function })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 156..180, focus_range: 167..175, name: \"test_foo\", kind: Function })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 182..233, focus_range: 214..228, name: \"test_full_path\", kind: Function })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 235..269, focus_range: 256..264, name: \"test_foo\", kind: Function })",
                    "(Bench, NavigationTarget { file_id: FileId(0), full_range: 271..293, focus_range: 283..288, name: \"bench\", kind: Function })",
                ]
            "#]] ,) ; } # [test] fn test_runnables_doc_test () { check (r#"
//- /lib.rs
$0
fn main() {}

/// ```
/// let x = 5;
/// ```
fn should_have_runnable() {}

/// ```edition2018
/// let x = 5;
/// ```
fn should_have_runnable_1() {}

/// ```
/// let z = 55;
/// ```
///
/// ```ignore
/// let z = 56;
/// ```
fn should_have_runnable_2() {}

/**
```rust
let z = 55;
```
*/
fn should_have_no_runnable_3() {}

/**
    ```rust
    let z = 55;
    ```
*/
fn should_have_no_runnable_4() {}

/// ```no_run
/// let z = 55;
/// ```
fn should_have_no_runnable() {}

/// ```ignore
/// let z = 55;
/// ```
fn should_have_no_runnable_2() {}

/// ```compile_fail
/// let z = 55;
/// ```
fn should_have_no_runnable_3() {}

/// ```text
/// arbitrary plain text
/// ```
fn should_have_no_runnable_4() {}

/// ```text
/// arbitrary plain text
/// ```
///
/// ```sh
/// $ shell code
/// ```
fn should_have_no_runnable_5() {}

/// ```rust,no_run
/// let z = 55;
/// ```
fn should_have_no_runnable_6() {}

/// ```
/// let x = 5;
/// ```
struct StructWithRunnable(String);

/// ```
/// let x = 5;
/// ```
impl StructWithRunnable {}

trait Test {
    fn test() -> usize {
        5usize
    }
}

/// ```
/// let x = 5;
/// ```
impl Test for StructWithRunnable {}
"# , expect ! [[r#"
                [
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 1..13, focus_range: 4..8, name: \"main\", kind: Function })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 15..74, name: \"should_have_runnable\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 76..148, name: \"should_have_runnable_1\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 150..254, name: \"should_have_runnable_2\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 256..320, name: \"should_have_no_runnable_3\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 322..398, name: \"should_have_no_runnable_4\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 900..965, name: \"StructWithRunnable\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 967..1024, focus_range: 1003..1021, name: \"impl\", kind: Impl })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 1088..1154, focus_range: 1133..1151, name: \"impl\", kind: Impl })",
                ]
            "#]] ,) ; } # [test] fn test_runnables_doc_test_in_impl () { check (r#"
//- /lib.rs
$0
fn main() {}

struct Data;
impl Data {
    /// ```
    /// let x = 5;
    /// ```
    fn foo() {}
}
"# , expect ! [[r#"
                [
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 1..13, focus_range: 4..8, name: \"main\", kind: Function })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 44..98, name: \"foo\" })",
                ]
            "#]] ,) ; } # [test] fn test_runnables_doc_test_in_impl_with_lifetime () { check (r#"
//- /lib.rs
$0
fn main() {}

struct Data<'a>;
impl Data<'a> {
    /// ```
    /// let x = 5;
    /// ```
    fn foo() {}
}
"# , expect ! [[r#"
                [
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 1..13, focus_range: 4..8, name: \"main\", kind: Function })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 52..106, name: \"foo\" })",
                ]
            "#]] ,) ; } # [test] fn test_runnables_doc_test_in_impl_with_lifetime_and_types () { check (r#"
//- /lib.rs
$0
fn main() {}

struct Data<'a, T, U>;
impl<T, U> Data<'a, T, U> {
    /// ```
    /// let x = 5;
    /// ```
    fn foo() {}
}
"# , expect ! [[r#"
                [
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 1..13, focus_range: 4..8, name: \"main\", kind: Function })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 70..124, name: \"foo\" })",
                ]
            "#]] ,) ; } # [test] fn test_runnables_doc_test_in_impl_with_const () { check (r#"
//- /lib.rs
$0
fn main() {}

struct Data<const N: usize>;
impl<const N: usize> Data<N> {
    /// ```
    /// let x = 5;
    /// ```
    fn foo() {}
}
"# , expect ! [[r#"
                [
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 1..13, focus_range: 4..8, name: \"main\", kind: Function })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 79..133, name: \"foo\" })",
                ]
            "#]] ,) ; } # [test] fn test_runnables_doc_test_in_impl_with_lifetime_types_and_const () { check (r#"
//- /lib.rs
$0
fn main() {}

struct Data<'a, T, const N: usize>;
impl<'a, T, const N: usize> Data<'a, T, N> {
    /// ```
    /// let x = 5;
    /// ```
    fn foo() {}
}
"# , expect ! [[r#"
                [
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 1..13, focus_range: 4..8, name: \"main\", kind: Function })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 100..154, name: \"foo\" })",
                ]
            "#]] ,) ; } # [test] fn test_runnables_module () { check (r#"
//- /lib.rs
$0
mod test_mod {
    #[test]
    fn test_foo1() {}
}
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 1..51, focus_range: 5..13, name: \"test_mod\", kind: Module, description: \"mod test_mod\" })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 20..49, focus_range: 35..44, name: \"test_foo1\", kind: Function })",
                ]
            "#]] ,) ; } # [test] fn only_modules_with_test_functions_or_more_than_one_test_submodule_have_runners () { check (r#"
//- /lib.rs
$0
mod root_tests {
    mod nested_tests_0 {
        mod nested_tests_1 {
            #[test]
            fn nested_test_11() {}

            #[test]
            fn nested_test_12() {}
        }

        mod nested_tests_2 {
            #[test]
            fn nested_test_2() {}
        }

        mod nested_tests_3 {}
    }

    mod nested_tests_4 {}
}
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 22..323, focus_range: 26..40, name: \"nested_tests_0\", kind: Module, description: \"mod nested_tests_0\" })",
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 51..192, focus_range: 55..69, name: \"nested_tests_1\", kind: Module, description: \"mod nested_tests_1\" })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 84..126, focus_range: 107..121, name: \"nested_test_11\", kind: Function })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 140..182, focus_range: 163..177, name: \"nested_test_12\", kind: Function })",
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 202..286, focus_range: 206..220, name: \"nested_tests_2\", kind: Module, description: \"mod nested_tests_2\" })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 235..276, focus_range: 258..271, name: \"nested_test_2\", kind: Function })",
                ]
            "#]] ,) ; } # [test] fn test_runnables_with_feature () { check (r#"
//- /lib.rs crate:foo cfg:feature=foo
$0
#[test]
#[cfg(feature = "foo")]
fn test_foo1() {}
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 0..51, name: \"_\", kind: Module })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 1..50, focus_range: 36..45, name: \"test_foo1\", kind: Function }, Atom(KeyValue { key: \"feature\", value: \"foo\" }))",
                ]
            "#]] ,) ; } # [test] fn test_runnables_with_features () { check (r#"
//- /lib.rs crate:foo cfg:feature=foo,feature=bar
$0
#[test]
#[cfg(all(feature = "foo", feature = "bar"))]
fn test_foo1() {}
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 0..73, name: \"_\", kind: Module })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 1..72, focus_range: 58..67, name: \"test_foo1\", kind: Function }, All([Atom(KeyValue { key: \"feature\", value: \"foo\" }), Atom(KeyValue { key: \"feature\", value: \"bar\" })]))",
                ]
            "#]] ,) ; } # [test] fn test_runnables_no_test_function_in_module () { check (r#"
//- /lib.rs
$0
mod test_mod {
    fn foo1() {}
}
"# , expect ! [[r#"
                []
            "#]] ,) ; } # [test] fn test_doc_runnables_impl_mod () { check (r#"
//- /lib.rs
mod foo;
//- /foo.rs
struct Foo;$0
impl Foo {
    /// ```
    /// let x = 5;
    /// ```
    fn foo() {}
}
        "# , expect ! [[r#"
                [
                    "(DocTest, NavigationTarget { file_id: FileId(1), full_range: 27..81, name: \"foo\" })",
                ]
            "#]] ,) ; } # [test] fn test_runnables_in_macro () { check (r#"
//- /lib.rs
$0
macro_rules! generate {
    () => {
        #[test]
        fn foo_test() {}
    }
}
macro_rules! generate2 {
    () => {
        mod tests2 {
            #[test]
            fn foo_test2() {}
        }
    }
}
macro_rules! generate_main {
    () => {
        fn main() {}
    }
}
mod tests {
    generate!();
}
generate2!();
generate_main!();
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 0..345, name: \"_\", kind: Module })",
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 282..312, focus_range: 286..291, name: \"tests\", kind: Module, description: \"mod tests\" })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 298..307, name: \"foo_test\", kind: Function })",
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 313..323, name: \"tests2\", kind: Module, description: \"mod tests2\" }, true)",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 313..323, name: \"foo_test2\", kind: Function }, true)",
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 327..341, name: \"main\", kind: Function })",
                ]
            "#]] ,) ; } # [test] fn big_mac () { check (r#"
//- /lib.rs
$0
macro_rules! foo {
    () => {
        mod foo_tests {
            #[test]
            fn foo0() {}
            #[test]
            fn foo1() {}
            #[test]
            fn foo2() {}
        }
    };
}
foo!();
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 210..214, name: \"foo_tests\", kind: Module, description: \"mod foo_tests\" }, true)",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 210..214, name: \"foo0\", kind: Function }, true)",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 210..214, name: \"foo1\", kind: Function }, true)",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 210..214, name: \"foo2\", kind: Function }, true)",
                ]
            "#]] ,) ; } # [test] fn dont_recurse_in_outline_submodules () { check (r#"
//- /lib.rs
$0
mod m;
//- /m.rs
mod tests {
    #[test]
    fn t() {}
}
"# , expect ! [[r#"
                []
            "#]] ,) ; } # [test] fn outline_submodule1 () { check (r#"
//- /lib.rs
$0
mod m;
//- /m.rs
#[test]
fn t0() {}
#[test]
fn t1() {}
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 1..7, focus_range: 5..6, name: \"m\", kind: Module, description: \"mod m\" })",
                ]
            "#]] ,) ; } # [test] fn outline_submodule2 () { check (r#"
//- /lib.rs
mod m;
//- /m.rs
$0
#[test]
fn t0() {}
#[test]
fn t1() {}
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(1), full_range: 0..39, name: \"m\", kind: Module })",
                    "(Test, NavigationTarget { file_id: FileId(1), full_range: 1..19, focus_range: 12..14, name: \"t0\", kind: Function })",
                    "(Test, NavigationTarget { file_id: FileId(1), full_range: 20..38, focus_range: 31..33, name: \"t1\", kind: Function })",
                ]
            "#]] ,) ; } # [test] fn attributed_module () { check (r#"
//- proc_macros: identity
//- /lib.rs
$0
#[proc_macros::identity]
mod module {
    #[test]
    fn t0() {}
    #[test]
    fn t1() {}
}
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 26..94, focus_range: 30..36, name: \"module\", kind: Module, description: \"mod module\" }, true)",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 43..65, focus_range: 58..60, name: \"t0\", kind: Function }, true)",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 70..92, focus_range: 85..87, name: \"t1\", kind: Function }, true)",
                ]
            "#]] ,) ; } # [test] fn find_no_tests () { check_tests (r#"
//- /lib.rs
fn foo$0() {  };
"# , expect ! [[r#"
                []
            "#]] ,) ; } # [test] fn find_direct_fn_test () { check_tests (r#"
//- /lib.rs
fn foo$0() { };

mod tests {
    #[test]
    fn foo_test() {
        super::foo()
    }
}
"# , expect ! [[r#"
                [
                    NavigationTarget {
                        file_id: FileId(
                            0,
                        ),
                        full_range: 31..85,
                        focus_range: 46..54,
                        name: "foo_test",
                        kind: Function,
                    },
                ]
            "#]] ,) ; } # [test] fn find_direct_struct_test () { check_tests (r#"
//- /lib.rs
struct Fo$0o;
fn foo(arg: &Foo) { };

mod tests {
    use super::*;

    #[test]
    fn foo_test() {
        foo(Foo);
    }
}
"# , expect ! [[r#"
                [
                    NavigationTarget {
                        file_id: FileId(
                            0,
                        ),
                        full_range: 71..122,
                        focus_range: 86..94,
                        name: "foo_test",
                        kind: Function,
                    },
                ]
            "#]] ,) ; } # [test] fn find_indirect_fn_test () { check_tests (r#"
//- /lib.rs
fn foo$0() { };

mod tests {
    use super::foo;

    fn check1() {
        check2()
    }

    fn check2() {
        foo()
    }

    #[test]
    fn foo_test() {
        check1()
    }
}
"# , expect ! [[r#"
                [
                    NavigationTarget {
                        file_id: FileId(
                            0,
                        ),
                        full_range: 133..183,
                        focus_range: 148..156,
                        name: "foo_test",
                        kind: Function,
                    },
                ]
            "#]] ,) ; } # [test] fn tests_are_unique () { check_tests (r#"
//- /lib.rs
fn foo$0() { };

mod tests {
    use super::foo;

    #[test]
    fn foo_test() {
        foo();
        foo();
    }

    #[test]
    fn foo2_test() {
        foo();
        foo();
    }

}
"# , expect ! [[r#"
                [
                    NavigationTarget {
                        file_id: FileId(
                            0,
                        ),
                        full_range: 52..115,
                        focus_range: 67..75,
                        name: "foo_test",
                        kind: Function,
                    },
                    NavigationTarget {
                        file_id: FileId(
                            0,
                        ),
                        full_range: 121..185,
                        focus_range: 136..145,
                        name: "foo2_test",
                        kind: Function,
                    },
                ]
            "#]] ,) ; } # [test] fn test_runnables_doc_test_in_impl_with_lifetime_type_const_value () { check (r#"
//- /lib.rs
$0
fn main() {}

struct Data<'a, A, const B: usize, C, const D: u32>;
impl<A, C, const D: u32> Data<'a, A, 12, C, D> {
    /// ```
    /// ```
    fn foo() {}
}
"# , expect ! [[r#"
                [
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 1..13, focus_range: 4..8, name: \"main\", kind: Function })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 121..156, name: \"foo\" })",
                ]
            "#]] ,) ; } # [test] fn doc_test_type_params () { check (r#"
//- /lib.rs
$0
struct Foo<T, U>;

/// ```
/// ```
impl<T, U> Foo<T, U> {
    /// ```rust
    /// ````
    fn t() {}
}

/// ```
/// ```
impl Foo<Foo<(), ()>, ()> {
    /// ```
    /// ```
    fn t() {}
}
"# , expect ! [[r#"
                [
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 20..103, focus_range: 47..56, name: \"impl\", kind: Impl })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 63..101, name: \"t\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 105..188, focus_range: 126..146, name: \"impl\", kind: Impl })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 153..186, name: \"t\" })",
                ]
            "#]] ,) ; } # [test] fn doc_test_macro_export_mbe () { check (r#"
//- /lib.rs
$0
mod foo;

//- /foo.rs
/// ```
/// fn foo() {
/// }
/// ```
#[macro_export]
macro_rules! foo {
    () => {

    };
}
"# , expect ! [[r#"
                []
            "#]] ,) ; check (r#"
//- /lib.rs
$0
/// ```
/// fn foo() {
/// }
/// ```
#[macro_export]
macro_rules! foo {
    () => {

    };
}
"# , expect ! [[r#"
                [
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 1..94, name: \"foo\" })",
                ]
            "#]] ,) ; } # [test] fn test_paths_with_raw_ident () { check (r#"
//- /lib.rs
$0
mod r#mod {
    #[test]
    fn r#fn() {}

    /// ```
    /// ```
    fn r#for() {}

    /// ```
    /// ```
    struct r#struct<r#type>(r#type);

    /// ```
    /// ```
    impl<r#type> r#struct<r#type> {
        /// ```
        /// ```
        fn r#fn() {}
    }

    enum r#enum {}
    impl r#struct<r#enum> {
        /// ```
        /// ```
        fn r#fn() {}
    }

    trait r#trait {}

    /// ```
    /// ```
    impl<T> r#trait for r#struct<T> {}
}
"# , expect ! [[r#"
                [
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 1..461, focus_range: 5..10, name: \"mod\", kind: Module, description: \"mod r#mod\" })",
                    "(Test, NavigationTarget { file_id: FileId(0), full_range: 17..41, focus_range: 32..36, name: \"r#fn\", kind: Function })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 47..84, name: \"r#for\", container_name: \"mod\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 90..146, name: \"r#struct\", container_name: \"mod\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 152..266, focus_range: 189..205, name: \"impl\", kind: Impl })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 216..260, name: \"r#fn\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 323..367, name: \"r#fn\" })",
                    "(DocTest, NavigationTarget { file_id: FileId(0), full_range: 401..459, focus_range: 445..456, name: \"impl\", kind: Impl })",
                ]
            "#]] ,) } # [test] fn exported_main_is_test_in_cfg_test_mod () { check (r#"
//- /lib.rs crate:foo cfg:test
$0
mod not_a_test_module_inline {
    #[export_name = "main"]
    fn exp_main() {}
}
#[cfg(test)]
mod test_mod_inline {
    #[export_name = "main"]
    fn exp_main() {}
}
mod not_a_test_module;
#[cfg(test)]
mod test_mod;
//- /not_a_test_module.rs
#[export_name = "main"]
fn exp_main() {}
//- /test_mod.rs
#[export_name = "main"]
fn exp_main() {}
"# , expect ! [[r#"
                [
                    "(Bin, NavigationTarget { file_id: FileId(0), full_range: 36..80, focus_range: 67..75, name: \"exp_main\", kind: Function })",
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 83..168, focus_range: 100..115, name: \"test_mod_inline\", kind: Module, description: \"mod test_mod_inline\" }, Atom(Flag(\"test\")))",
                    "(TestMod, NavigationTarget { file_id: FileId(0), full_range: 192..218, focus_range: 209..217, name: \"test_mod\", kind: Module, description: \"mod test_mod\" }, Atom(Flag(\"test\")))",
                ]
            "#]] ,) } }
};
}
