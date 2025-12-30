// Generated macro for test (module)
macro_rules! Depcrate_parse_rstesttest {
() => {
// Module: crate::parse::rstest
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: test :: { assert_eq , * } ; use rstest_test :: assert_in ; mod parse_rstest_data { use super :: assert_eq ; use super :: * ; fn parse_rstest_data < S : AsRef < str > > (fixtures : S) -> RsTestData { parse_meta (fixtures) } # [test] fn one_arg () { let fixtures = parse_rstest_data ("my_fixture(42)") ; let expected = RsTestData { items : vec ! [fixture ("my_fixture" , & ["42"]) . into ()] , } ; assert_eq ! (expected , fixtures) ; } } # [test] fn should_check_all_timeout_to_catch_the_right_errors () { let mut item_fn = r#"
            #[timeout(<some>)]
            #[timeout(42)]
            #[timeout]
            #[timeout(Duration::from_millis(20))]
            fn test_fn(#[case] arg: u32) {
            }
        "# . ast () ; let mut info = RsTestInfo :: default () ; let errors = info . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_eq ! (2 , errors . len ()) ; } # [cfg (feature = "async-timeout")] # [test] fn should_parse_async_timeout () { let mut item_fn = r#"
            #[timeout(Duration::from_millis(20))]
            async fn test_fn(#[case] arg: u32) {
            }
        "# . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; } # [cfg (not (feature = "async-timeout"))] # [test] fn should_return_error_for_async_timeout () { let mut item_fn = r#"
            #[timeout(Duration::from_millis(20))]
            async fn test_fn(#[case] arg: u32) {
            }
        "# . ast () ; let mut info = RsTestInfo :: default () ; let errors = info . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_eq ! (1 , errors . len ()) ; assert ! (format ! ("{:?}" , errors) . contains ("async-timeout feature")) } fn parse_rstest < S : AsRef < str > > (rstest_data : S) -> RsTestInfo { parse_meta (rstest_data) } mod no_cases { use std :: collections :: HashSet ; use super :: { assert_eq , * } ; # [test] fn happy_path () { let data = parse_rstest (r#"my_fixture(42, "other"), other(vec![42])
            :: trace :: no_trace(some)"# ,) ; let expected = RsTestInfo { data : vec ! [fixture ("my_fixture" , & ["42" , r#""other""#]) . into () , fixture ("other" , & ["vec![42]"]) . into () ,] . into () , attributes : Attributes { attributes : vec ! [Attribute :: attr ("trace") , Attribute :: tagged ("no_trace" , vec ! ["some"]) ,] , } . into () , .. Default :: default () } ; assert_eq ! (expected , data) ; } mod fixture_extraction { use super :: { assert_eq , * } ; # [test] fn rename () { let data = parse_rstest (r#"long_fixture_name(42, "other") as short, sub_module::fix as f, simple as s, no_change()"# ,) ; let expected = RsTestInfo { data : vec ! [fixture ("short" , & ["42" , r#""other""#]) . with_resolve ("long_fixture_name") . into () , fixture ("f" , & []) . with_resolve ("sub_module::fix") . into () , fixture ("s" , & []) . with_resolve ("simple") . into () , fixture ("no_change" , & []) . into () ,] . into () , .. Default :: default () } ; assert_eq ! (expected , data) ; } # [test] fn rename_with_attributes () { let mut item_fn = r#"
                    fn test_fn(
                        #[from(long_fixture_name)] 
                        #[with(42, "other")] short: u32, 
                        #[from(simple)]
                        s: &str,
                        #[from(sub_module::fix)]
                        f: u32,
                        no_change: i32) {
                    }
                    "# . ast () ; let expected = RsTestInfo { data : vec ! [fixture ("short" , & ["42" , r#""other""#]) . with_resolve ("long_fixture_name") . into () , fixture ("s" , & []) . with_resolve ("simple") . into () , fixture ("f" , & []) . with_resolve ("sub_module::fix") . into () ,] . into () , .. Default :: default () } ; let mut data = RsTestInfo :: default () ; data . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert_eq ! (expected , data) ; } # [test] fn defined_via_with_attributes () { let mut item_fn = r#"
                    fn test_fn(#[with(42, "other")] my_fixture: u32, #[with(vec![42])] other: &str) {
                    }
                    "# . ast () ; let expected = RsTestInfo { data : vec ! [fixture ("my_fixture" , & ["42" , r#""other""#]) . into () , fixture ("other" , & ["vec![42]"]) . into () ,] . into () , .. Default :: default () } ; let mut data = RsTestInfo :: default () ; data . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert_eq ! (expected , data) ; } } # [test] fn empty_fixtures () { let data = parse_rstest (r#"::trace::no_trace(some)"#) ; let expected = RsTestInfo { attributes : Attributes { attributes : vec ! [Attribute :: attr ("trace") , Attribute :: tagged ("no_trace" , vec ! ["some"]) ,] , } . into () , .. Default :: default () } ; assert_eq ! (expected , data) ; } # [test] fn empty_attributes () { let data = parse_rstest (r#"my_fixture(42, "other")"#) ; let expected = RsTestInfo { data : vec ! [fixture ("my_fixture" , & ["42" , r#""other""#]) . into ()] . into () , .. Default :: default () } ; assert_eq ! (expected , data) ; } # [test] fn extract_notrace_args_attribute () { let mut item_fn = r#"
            fn test_fn(#[notrace] a: u32, #[something_else] b: &str, #[notrace] c: i32) {
            }
            "# . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; info . attributes . add_trace (ident ("trace")) ; assert ! (! info . attributes . trace_me (& pat ("a"))) ; assert ! (info . attributes . trace_me (& pat ("b"))) ; assert ! (! info . attributes . trace_me (& pat ("c"))) ; let b_args = item_fn . sig . inputs . into_iter () . nth (1) . and_then (| id | match id { syn :: FnArg :: Typed (arg) => Some (arg . attrs) , _ => None , }) . unwrap () ; assert_eq ! (attrs ("#[something_else]") , b_args) ; } # [rstest] fn extract_future () { let mut item_fn = "fn f(#[future] a: u32, b: u32) {}" . ast () ; let expected = "fn f(a: u32, b: u32) {}" . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert_eq ! (item_fn , expected) ; assert ! (info . arguments . is_future (& pat ("a"))) ; assert ! (! info . arguments . is_future (& pat ("b"))) ; } # [rstest] fn extract_context () { let mut item_fn = "fn f(#[context] c: Context, #[context] other: Context, more: u32) {}" . ast () ; let expected = "fn f(c: Context, other: Context, more: u32) {}" . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert_eq ! (item_fn , expected) ; assert_eq ! (info . arguments . contexts () . cloned () . collect ::< HashSet < _ >> () , vec ! [pat ("c") , pat ("other")] . into_iter () . collect ::< HashSet < _ >> ()) ; } # [test] fn extract_test_attr () { let mut item_fn = "#[test_attr(some_valid_data)] fn f() {}" . ast () ; let expected = "fn f() {}" . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert_eq ! (item_fn , expected) ; assert_eq ! (info . arguments . test_attr () . cloned () , Some (attr ("#[some_valid_data]") . into ()) ,) ; } } mod parametrize_cases { use super :: { assert_eq , * } ; # [test] fn one_simple_case_one_arg () { let data = parse_rstest (r#"arg, case(42)"#) . data ; let args = data . case_args () . collect :: < Vec < _ > > () ; let cases = data . cases () . collect :: < Vec < _ > > () ; assert_eq ! (1 , args . len ()) ; assert_eq ! (1 , cases . len ()) ; assert_eq ! ("arg" , & args [0] . display_code ()) ; assert_eq ! (to_args ! (["42"]) , cases [0] . args ()) } # [test] fn happy_path () { let info = parse_rstest (r#"
                my_fixture(42,"foo"),
                arg1, arg2, arg3,
                case(1,2,3),
                case(11,12,13),
                case(21,22,23)
            "# ,) ; let data = info . data ; let fixtures = data . fixtures () . cloned () . collect :: < Vec < _ > > () ; assert_eq ! (vec ! [fixture ("my_fixture" , & ["42" , r#""foo""#])] , fixtures) ; assert_eq ! (to_strs ! (vec ! ["arg1" , "arg2" , "arg3"]) , data . case_args () . map (DisplayCode :: display_code) . collect ::< Vec < _ >> ()) ; let cases = data . cases () . collect :: < Vec < _ > > () ; assert_eq ! (3 , cases . len ()) ; assert_eq ! (to_args ! (["1" , "2" , "3"]) , cases [0] . args ()) ; assert_eq ! (to_args ! (["11" , "12" , "13"]) , cases [1] . args ()) ; assert_eq ! (to_args ! (["21" , "22" , "23"]) , cases [2] . args ()) ; } mod defined_via_with_attributes { use super :: { assert_eq , * } ; # [test] fn one_case () { let mut item_fn = r#"
                #[case::first(42, "first")]
                fn test_fn(#[case] arg1: u32, #[case] arg2: &str) {
                }
                "# . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; let case_args = info . data . case_args () . cloned () . collect :: < Vec < _ > > () ; let cases = info . data . cases () . cloned () . collect :: < Vec < _ > > () ; assert_eq ! (to_pats ! (["arg1" , "arg2"]) , case_args) ; assert_eq ! (vec ! [TestCase :: from_iter (["42" , r#""first""#] . iter ()) . with_description ("first") ,] , cases) ; } # [test] fn destruct_case () { let mut item_fn : ItemFn = r#"
                #[case::destruct(T::new(2, 21))]
                fn test_fn(#[case] T{a, b}: T) {
                }
                "# . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; let case_args = info . data . case_args () . cloned () . collect :: < Vec < _ > > () ; let cases = info . data . cases () . cloned () . collect :: < Vec < _ > > () ; assert_eq ! (to_fnargs ! (["T{a, b}: T"]) , item_fn . sig . inputs . into_iter () . collect ::< Vec < _ >> ()) ; assert_eq ! (to_pats ! (["T{a, b}"]) , case_args) ; assert_eq ! (vec ! [TestCase :: from_iter (["T::new(2, 21)"] . iter ()) . with_description ("destruct") ,] , cases) ; assert_eq ! (info . arguments . inner_pat (& pat ("T{a, b}")) , & pat ("__destruct_1")) ; } # [test] fn parse_tuple_value () { let mut item_fn = r#"
                #[case(42, (24, "first"))]
                fn test_fn(#[case] arg1: u32, #[case] tupled: (u32, &str)) {
                }
                "# . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; let cases = info . data . cases () . cloned () . collect :: < Vec < _ > > () ; assert_eq ! (vec ! [TestCase :: from_iter (["42" , r#"(24, "first")"#] . iter ()) ,] , cases) ; } # [test] fn more_cases () { let mut item_fn = r#"
                #[case::first(42, "first")]
                #[case(24, "second")]
                #[case::third(0, "third")]
                fn test_fn(#[case] arg1: u32, #[case] arg2: &str) {
                }
                "# . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; let case_args = info . data . case_args () . cloned () . collect :: < Vec < _ > > () ; let cases = info . data . cases () . cloned () . collect :: < Vec < _ > > () ; assert_eq ! (to_pats ! (["arg1" , "arg2"]) , case_args) ; assert_eq ! (vec ! [TestCase :: from_iter (["42" , r#""first""#] . iter ()) . with_description ("first") , TestCase :: from_iter (["24" , r#""second""#] . iter ()) , TestCase :: from_iter (["0" , r#""third""#] . iter ()) . with_description ("third") ,] , cases) ; } # [test] fn should_collect_attributes () { let mut item_fn = r#"
                    #[first]
                    #[first2(42)]
                    #[case(42)]
                    #[second]
                    #[case(24)]
                    #[global]
                    fn test_fn(#[case] arg: u32) {
                    }
                "# . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; let cases = info . data . cases () . cloned () . collect :: < Vec < _ > > () ; assert_eq ! (vec ! [TestCase :: from_iter (["42"] . iter ()) . with_attrs (attrs ("
                                #[first]
                                #[first2(42)]
                            ")) , TestCase :: from_iter (["24"] . iter ()) . with_attrs (attrs ("
                            #[second]
                        ")) ,] , cases) ; } # [test] fn should_consume_all_used_attributes () { let mut item_fn = r#"
                    #[first]
                    #[first2(42)]
                    #[case(42)]
                    #[second]
                    #[case(24)]
                    #[global]
                    fn test_fn(#[case] arg: u32) {
                    }
                "# . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; assert_eq ! (item_fn . attrs , attrs ("
                        #[global]
                        ")) ; assert ! (! format ! ("{:?}" , item_fn) . contains ("case")) ; } # [test] fn should_report_all_errors () { let mut item_fn = r#"
                    #[case(#case_error#)]
                    fn test_fn(#[case] arg: u32, #[with(#fixture_error#)] err_fixture: u32) {
                    }
                "# . ast () ; let mut info = RsTestInfo :: default () ; let errors = info . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_eq ! (2 , errors . len ()) ; } } # [test] fn should_accept_comma_at_the_end_of_cases () { let data = parse_rstest (r#"
                arg,
                case(42),
            "# ,) . data ; let args = data . case_args () . collect :: < Vec < _ > > () ; let cases = data . cases () . collect :: < Vec < _ > > () ; assert_eq ! (1 , args . len ()) ; assert_eq ! (1 , cases . len ()) ; assert_eq ! ("arg" , & args [0] . display_code ()) ; assert_eq ! (to_args ! (["42"]) , cases [0] . args ()) } # [test] # [should_panic] fn should_not_accept_invalid_separator_from_args_and_cases () { parse_rstest (r#"
                ret
                case::should_success(Ok(())),
                case::should_fail(Err("Return Error"))
            "# ,) ; } # [test] fn case_could_be_arg_name () { let data = parse_rstest (r#"
                case,
                case(42)
            "# ,) . data ; assert_eq ! ("case" , & data . case_args () . next () . unwrap () . display_code ()) ; let cases = data . cases () . collect :: < Vec < _ > > () ; assert_eq ! (1 , cases . len ()) ; assert_eq ! (to_args ! (["42"]) , cases [0] . args ()) ; } # [test] fn should_reject_case_args_marked_more_than_once () { let mut item_fn = r#"
                    #[case(42)]
                    fn test_fn(#[case] #[case] arg: u32) {
                    }
                "# . ast () ; let mut info = RsTestInfo :: default () ; let errors = info . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_eq ! (1 , errors . len ()) ; assert_in ! (errors [0] . to_string () , "more than once") ; } } mod matrix_cases { use super :: { assert_eq , * } ; # [test] fn happy_path () { let info = parse_rstest (r#"
                    expected => [12, 34 * 2],
                    input => [format!("aa_{}", 2), "other"],
                "# ,) ; let value_ranges = info . data . list_values () . collect :: < Vec < _ > > () ; assert_eq ! (2 , value_ranges . len ()) ; assert_eq ! (to_args ! (["12" , "34 * 2"]) , value_ranges [0] . args ()) ; assert_eq ! (to_args ! ([r#"format!("aa_{}", 2)"# , r#""other""#]) , value_ranges [1] . args ()) ; assert_eq ! (info . attributes , Default :: default ()) ; } # [test] fn should_parse_attributes_too () { let info = parse_rstest (r#"
                                        a => [12, 24, 42]
                                        ::trace
                                    "# ,) ; assert_eq ! (info . attributes , Attributes { attributes : vec ! [Attribute :: attr ("trace")] } . into ()) ; } # [test] fn should_parse_injected_fixtures_too () { let info = parse_rstest (r#"
                a => [12, 24, 42],
                fixture_1(42, "foo"),
                fixture_2("bar")
                "# ,) ; let fixtures = info . data . fixtures () . cloned () . collect :: < Vec < _ > > () ; assert_eq ! (vec ! [fixture ("fixture_1" , & ["42" , r#""foo""#]) , fixture ("fixture_2" , & [r#""bar""#])] , fixtures) ; } # [test] # [should_panic (expected = "should not be empty")] fn should_not_compile_if_empty_expression_slice () { parse_rstest (r#"
                invalid => []
                "# ,) ; } mod defined_via_with_attributes { use super :: { assert_eq , * } ; # [test] fn one_arg () { let mut item_fn = r#"
                fn test_fn(#[values(1, 2, 1+2)] arg1: u32, #[values(format!("a"), "b b".to_owned(), String::new())] arg2: String) {
                }
                "# . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; let list_values = info . data . list_values () . cloned () . collect :: < Vec < _ > > () ; assert_eq ! (2 , list_values . len ()) ; assert_eq ! (to_args ! (["1" , "2" , "1+2"]) , list_values [0] . args ()) ; assert_eq ! (to_args ! ([r#"format!("a")"# , r#""b b".to_owned()"# , "String::new()"]) , list_values [1] . args ()) ; } # [test] fn destruct () { let mut item_fn = r#"
                fn test_fn(#[values(S(1,2), S(3,4))] S(a,b): S, #[values(T::new("a", "b"), T{s: "a" ,t: "c" })] T{s, t}: T) {
                }
                "# . ast () ; let mut info = RsTestInfo :: default () ; info . extend_with_function_attrs (& mut item_fn) . unwrap () ; let list_values = info . data . list_values () . cloned () . collect :: < Vec < _ > > () ; assert_eq ! (to_fnargs ! (["S(a, b): S" , "T{s, t}: T"]) , item_fn . sig . inputs . into_iter () . collect ::< Vec < _ >> ()) ; assert_eq ! (2 , list_values . len ()) ; assert_eq ! (list_values [0] . arg , pat ("S(a, b)")) ; assert_eq ! (to_args ! (["S(1,2)" , "S(3,4)"]) , list_values [0] . args ()) ; assert_eq ! (list_values [1] . arg , pat ("T{s, t}")) ; assert_eq ! (to_args ! ([r#"T::new("a", "b")"# , r#"T{s: "a" ,t: "c" }"#]) , list_values [1] . args ()) ; assert_eq ! (info . arguments . inner_pat (& pat ("S(a, b)")) , & pat ("__destruct_1")) ; assert_eq ! (info . arguments . inner_pat (& pat ("T{s, t}")) , & pat ("__destruct_2")) ; } } # [test] fn should_reject_values_attribute_marked_more_than_once () { let mut item_fn = r#"
                fn test_fn(#[values(1, 2, 1+2)] #[values(1, 2, 1+2)] arg1: u32, ) {
                }
                "# . ast () ; let mut info = RsTestInfo :: default () ; let errors = info . extend_with_function_attrs (& mut item_fn) . unwrap_err () ; assert_eq ! (1 , errors . len ()) ; assert_in ! (errors [0] . to_string () , "more than once") ; } } mod integrated { use super :: { assert_eq , * } ; # [test] fn should_parse_fixture_cases_and_matrix_in_any_order () { let data = parse_rstest (r#"
                u,
                m => [1, 2],
                case(42, A{}, D{}),
                a,
                case(43, A{}, D{}),
                the_fixture(42),
                mm => ["f", "oo", "BAR"],
                d
            "# ,) . data ; let fixtures = data . fixtures () . cloned () . collect :: < Vec < _ > > () ; assert_eq ! (vec ! [fixture ("the_fixture" , & ["42"])] , fixtures) ; assert_eq ! (to_strs ! (vec ! ["u" , "a" , "d"]) , data . case_args () . map (DisplayCode :: display_code) . collect ::< Vec < _ >> ()) ; let cases = data . cases () . collect :: < Vec < _ > > () ; assert_eq ! (2 , cases . len ()) ; assert_eq ! (to_args ! (["42" , "A{}" , "D{}"]) , cases [0] . args ()) ; assert_eq ! (to_args ! (["43" , "A{}" , "D{}"]) , cases [1] . args ()) ; let value_ranges = data . list_values () . collect :: < Vec < _ > > () ; assert_eq ! (2 , value_ranges . len ()) ; assert_eq ! (to_args ! (["1" , "2"]) , value_ranges [0] . args ()) ; assert_eq ! (to_args ! ([r#""f""# , r#""oo""# , r#""BAR""#]) , value_ranges [1] . args ()) ; } } }
};
}
