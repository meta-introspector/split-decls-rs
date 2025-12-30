// Generated macro for extend (module)
macro_rules! Depcrate_parse_fixtureextend {
() => {
// Module: crate::parse::fixture
// Provides: {"extend"}
// Dependencies: {}
# [cfg (test)] mod extend { use super :: * ; use crate :: test :: { assert_eq , * } ; use syn :: ItemFn ; mod should { use super :: { assert_eq , * } ; # [test] fn use_with_attributes () { let to_parse = r#"
                fn my_fix(#[with(2)] f1: &str, #[with(vec![1,2], "s")] f2: u32) {}
            "# ; let mut item_fn : ItemFn = to_parse . ast () ; let mut info = FixtureInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; let expected = FixtureInfo { data : vec ! [fixture ("f1" , & ["2"]) . into () , fixture ("f2" , & ["vec![1,2]" , r#""s""#]) . into () ,] . into () , .. Default :: default () } ; assert ! (! format ! ("{:?}" , item_fn) . contains ("with")) ; assert_eq ! (expected , info) ; } # [test] fn rename_with_attributes () { let mut item_fn = r#"
                    fn test_fn(
                        #[from(long_fixture_name)] 
                        #[with(42, "other")] short: u32, 
                        #[from(sub_module::fix)]
                        f: u32,
                        #[from(simple)]
                        s: &str,
                        no_change: i32) {
                    }
                    "# . ast () ; let expected = FixtureInfo { data : vec ! [fixture ("short" , & ["42" , r#""other""#]) . with_resolve ("long_fixture_name") . into () , fixture ("f" , & []) . with_resolve ("sub_module::fix") . into () , fixture ("s" , & []) . with_resolve ("simple") . into () ,] . into () , .. Default :: default () } ; let mut data = FixtureInfo :: default () ; data . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert_eq ! (expected , data) ; } # [test] fn use_default_values_attributes () { let to_parse = r#"
                fn my_fix(#[default(2)] f1: &str, #[default((vec![1,2], "s"))] f2: (Vec<u32>, &str)) {}
            "# ; let mut item_fn : ItemFn = to_parse . ast () ; let mut info = FixtureInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; let expected = FixtureInfo { data : vec ! [arg_value ("f1" , "2") . into () , arg_value ("f2" , r#"(vec![1,2], "s")"#) . into () ,] . into () , .. Default :: default () } ; assert ! (! format ! ("{:?}" , item_fn) . contains ("default")) ; assert_eq ! (expected , info) ; } # [test] fn find_default_return_type () { let mut item_fn : ItemFn = r#"
                #[simple]
                #[first(comp)]
                #[second::default]
                #[default(impl Iterator<Item=(u32, i32)>)]
                #[last::more]
                fn my_fix<I, J>(f1: I, f2: J) -> impl Iterator<Item=(I, J)> {}
            "# . ast () ; let mut info = FixtureInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert_eq ! (info . attributes . extract_default_type () , Some (parse_quote ! { -> impl Iterator < Item = (u32 , i32) > })) ; assert_eq ! (attrs ("#[simple]#[first(comp)]#[second::default]#[last::more]") , item_fn . attrs) ; } # [test] fn find_partials_return_type () { let mut item_fn : ItemFn = r#"
                #[simple]
                #[first(comp)]
                #[second::default]
                #[partial_1(impl Iterator<Item=(u32, J, K)>)]
                #[partial_2(impl Iterator<Item=(u32, i32, K)>)]
                #[last::more]
                fn my_fix<I, J, K>(f1: I, f2: J, f3: K) -> impl Iterator<Item=(I, J, K)> {}
            "# . ast () ; let mut info = FixtureInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert_eq ! (info . attributes . extract_partial_type (1) , Some (parse_quote ! { -> impl Iterator < Item = (u32 , J , K) > })) ; assert_eq ! (info . attributes . extract_partial_type (2) , Some (parse_quote ! { -> impl Iterator < Item = (u32 , i32 , K) > })) ; assert_eq ! (attrs ("#[simple]#[first(comp)]#[second::default]#[last::more]") , item_fn . attrs) ; } # [test] fn find_once_attribute () { let mut item_fn : ItemFn = r#"
                #[simple]
                #[first(comp)]
                #[second::default]
                #[once]
                #[last::more]
                fn my_fix<I, J, K>(f1: I, f2: J, f3: K) -> impl Iterator<Item=(I, J, K)> {}
            "# . ast () ; let mut info = FixtureInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert ! (info . arguments . is_once ()) ; } # [test] fn no_once_attribute () { let mut item_fn : ItemFn = r#"
                fn my_fix<I, J, K>(f1: I, f2: J, f3: K) -> impl Iterator<Item=(I, J, K)> {}
            "# . ast () ; let mut info = FixtureInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert ! (! info . arguments . is_once ()) ; } # [rstest] fn extract_future () { let mut item_fn = "fn f(#[future] a: u32, b: u32) {}" . ast () ; let expected = "fn f(a: u32, b: u32) {}" . ast () ; let mut info = FixtureInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert_eq ! (item_fn , expected) ; assert ! (info . arguments . is_future (& pat ("a"))) ; assert ! (! info . arguments . is_future (& pat ("b"))) ; } mod raise_error { use super :: { assert_eq , * } ; use rstest_test :: assert_in ; # [test] fn for_invalid_expressions () { let mut item_fn : ItemFn = r#"
                fn my_fix(#[with(valid)] f1: &str, #[with(with(,.,))] f2: u32, #[with(with(use))] f3: u32) {}
                "# . ast () ; let errors = FixtureInfo :: default () . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_eq ! (2 , errors . len ()) ; } # [test] fn for_invalid_default_type () { let mut item_fn : ItemFn = r#"
                    #[default(no<valid::>type)]
                    fn my_fix<I>() -> I {}
                "# . ast () ; let errors = FixtureInfo :: default () . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_eq ! (1 , errors . len ()) ; } # [test] fn with_used_more_than_once () { let mut item_fn : ItemFn = r#"
                    fn my_fix(#[with(1)] #[with(2)] fixture1: &str, #[with(1)] #[with(2)] #[with(3)] fixture2: &str) {}
                "# . ast () ; let errors = FixtureInfo :: default () . extend_with_function_attrs (& mut item_fn) . err () . unwrap_or_default () ; assert_eq ! (3 , errors . len ()) ; } # [test] fn fixture_destruct_without_from () { let mut item_fn : ItemFn = r#"
                    fn my_fix(#[with(1)] T{a}: T) {}
                "# . ast () ; let errors = FixtureInfo :: default () . extend_with_function_attrs (& mut item_fn) . err () . unwrap_or_default () ; assert_in ! (errors [0] . to_string () , "destruct") ; } # [test] fn from_used_more_than_once () { let mut item_fn : ItemFn = r#"
                    fn my_fix(#[from(a)] #[from(b)] fixture1: &str, #[from(c)] #[from(d)] #[from(e)] fixture2: &str) {}
                "# . ast () ; let errors = FixtureInfo :: default () . extend_with_function_attrs (& mut item_fn) . err () . unwrap_or_default () ; assert_eq ! (3 , errors . len ()) ; } # [test] fn future_is_used_more_than_once () { let mut item_fn : ItemFn = r#"
                    fn my_fix(#[future] #[future] fixture1: u32) {}
                "# . ast () ; let errors = FixtureInfo :: default () . extend_with_function_attrs (& mut item_fn) . err () . unwrap_or_default () ; assert_eq ! (1 , errors . len ()) ; assert_in ! (errors [0] . to_string () , "more than once") ; } # [test] fn default_used_more_than_once () { let mut item_fn : ItemFn = r#"
                    fn my_fix(#[default(2)] #[default(3)] f1: u32) {}
                "# . ast () ; let errors = FixtureInfo :: default () . extend_with_function_attrs (& mut item_fn) . err () . unwrap_or_default () ; assert_eq ! (1 , errors . len ()) ; assert_in ! (errors [0] . to_string () , "more than once") ; } # [test] fn if_once_is_defined_more_than_once () { let mut item_fn : ItemFn = r#"
                    #[once]
                    #[once]
                    fn my_fix<I>() -> I {}
                    "# . ast () ; let mut info = FixtureInfo :: default () ; let error = info . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_in ! (format ! ("{:?}" , error) . to_lowercase () , "cannot use #[once] more than once") ; } # [test] fn if_default_is_defined_more_than_once () { let mut item_fn : ItemFn = r#"
                    #[default(u32)]
                    #[default(u32)]
                    fn my_fix<I>() -> I {}
                    "# . ast () ; let mut info = FixtureInfo :: default () ; let error = info . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_in ! (format ! ("{:?}" , error) . to_lowercase () , "cannot use #[default] more than once") ; } # [test] fn for_invalid_partial_type () { let mut item_fn : ItemFn = r#"
                    #[partial_1(no<valid::>type)]
                    fn my_fix<I>(x: I, y: u32) -> I {}
                "# . ast () ; let errors = FixtureInfo :: default () . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_eq ! (1 , errors . len ()) ; } # [test] fn if_partial_is_not_correct () { let mut item_fn : ItemFn = r#"
                    #[partial_not_a_number(u32)]
                    fn my_fix<I, J>(f1: I, f2: &str) -> I {}
                    "# . ast () ; let mut info = FixtureInfo :: default () ; let error = info . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_in ! (format ! ("{:?}" , error) . to_lowercase () , "invalid partial syntax") ; } } } }
};
}
