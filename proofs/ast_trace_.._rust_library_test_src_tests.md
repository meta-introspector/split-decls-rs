# AST Trace: ../rust/library/test/src/tests.rs

Generated 36 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=13

```rust
use super::*;
use crate::{
    console::OutputLocation,
    formatters::PrettyFormatter,
    test::{
        MetricMap,
        // FIXME (introduced by #65251)
        // ShouldPanic, StaticTestName, TestDesc, TestDescAndFn, TestOpts, TestTimeOptions,
        // TestType, TrFailedMsg, TrIgnored, TrOk,
        parse_opts,
    },
    time::{TestTimeOptions, TimeThreshold},
};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=new | COMPLEXITY=5 | LINES=26

```rust
impl TestOpts {
    fn new() -> TestOpts {
        TestOpts {
            list: false,
            filters: vec![],
            filter_exact: false,
            force_run_in_process: false,
            exclude_should_panic: false,
            run_ignored: RunIgnored::No,
            run_tests: false,
            bench_benchmarks: false,
            logfile: None,
            nocapture: false,
            color: AutoColor,
            format: OutputFormat::Pretty,
            shuffle: false,
            shuffle_seed: None,
            test_threads: None,
            skip: vec![],
            time_options: None,
            options: Options::new(),
            fail_fast: false,
        }
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=one_ignored_one_unignored_test | COMPLEXITY=8 | LINES=39

```rust
fn one_ignored_one_unignored_test() -> Vec<TestDescAndFn> {
    vec![
        TestDescAndFn {
            desc: TestDesc {
                name: StaticTestName("1"),
                ignore: true,
                ignore_message: None,
                source_file: "",
                start_line: 0,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                should_panic: ShouldPanic::No,
                compile_fail: false,
                no_run: false,
                test_type: TestType::Unknown,
            },
            testfn: DynTestFn(Box::new(move || Ok(()))),
        },
        TestDescAndFn {
            desc: TestDesc {
                name: StaticTestName("2"),
                ignore: false,
                ignore_message: None,
                source_file: "",
                start_line: 0,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                should_panic: ShouldPanic::No,
                compile_fail: false,
                no_run: false,
                test_type: TestType::Unknown,
            },
            testfn: DynTestFn(Box::new(move || Ok(()))),
        },
    ]
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=do_not_run_ignored_tests | COMPLEXITY=6 | LINES=28

```rust
#[test]
fn do_not_run_ignored_tests() {
    fn f() -> Result<(), String> {
        panic!();
    }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: true,
            ignore_message: None,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::No,
            compile_fail: false,
            no_run: false,
            test_type: TestType::Unknown,
        },
        testfn: DynTestFn(Box::new(f)),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, TestId(0), desc, RunStrategy::InProcess, tx);
    let result = rx.recv().unwrap().result;
    assert_ne!(result, TrOk);
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=ignored_tests_result_in_ignored | COMPLEXITY=6 | LINES=28

```rust
#[test]
fn ignored_tests_result_in_ignored() {
    fn f() -> Result<(), String> {
        Ok(())
    }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: true,
            ignore_message: None,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::No,
            compile_fail: false,
            no_run: false,
            test_type: TestType::Unknown,
        },
        testfn: DynTestFn(Box::new(f)),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, TestId(0), desc, RunStrategy::InProcess, tx);
    let result = rx.recv().unwrap().result;
    assert_eq!(result, TrIgnored);
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=test_should_panic | COMPLEXITY=6 | LINES=29

```rust
#[test]
#[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
fn test_should_panic() {
    fn f() -> Result<(), String> {
        panic!();
    }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            ignore_message: None,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::Yes,
            compile_fail: false,
            no_run: false,
            test_type: TestType::Unknown,
        },
        testfn: DynTestFn(Box::new(f)),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, TestId(0), desc, RunStrategy::InProcess, tx);
    let result = rx.recv().unwrap().result;
    assert_eq!(result, TrOk);
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=test_should_panic_good_message | COMPLEXITY=6 | LINES=29

```rust
#[test]
#[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
fn test_should_panic_good_message() {
    fn f() -> Result<(), String> {
        panic!("an error message");
    }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            ignore_message: None,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::YesWithMessage("error message"),
            compile_fail: false,
            no_run: false,
            test_type: TestType::Unknown,
        },
        testfn: DynTestFn(Box::new(f)),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, TestId(0), desc, RunStrategy::InProcess, tx);
    let result = rx.recv().unwrap().result;
    assert_eq!(result, TrOk);
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=test_should_panic_bad_message | COMPLEXITY=7 | LINES=34

```rust
#[test]
#[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
fn test_should_panic_bad_message() {
    use crate::tests::TrFailedMsg;
    fn f() -> Result<(), String> {
        panic!("an error message");
    }
    let expected = "foobar";
    let failed_msg = r#"panic did not contain expected string
      panic message: "an error message"
 expected substring: "foobar""#;
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            ignore_message: None,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::YesWithMessage(expected),
            compile_fail: false,
            no_run: false,
            test_type: TestType::Unknown,
        },
        testfn: DynTestFn(Box::new(f)),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, TestId(0), desc, RunStrategy::InProcess, tx);
    let result = rx.recv().unwrap().result;
    assert_eq!(result, TrFailedMsg(failed_msg.to_string()));
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=test_should_panic_non_string_message_type | COMPLEXITY=8 | LINES=39

```rust
#[test]
#[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
fn test_should_panic_non_string_message_type() {
    use std::any::TypeId;

    use crate::tests::TrFailedMsg;
    fn f() -> Result<(), String> {
        std::panic::panic_any(1i32);
    }
    let expected = "foobar";
    let failed_msg = format!(
        r#"expected panic with string value,
 found non-string value: `{:?}`
     expected substring: "foobar""#,
        TypeId::of::<i32>()
    );
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            ignore_message: None,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::YesWithMessage(expected),
            compile_fail: false,
            no_run: false,
            test_type: TestType::Unknown,
        },
        testfn: DynTestFn(Box::new(f)),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, TestId(0), desc, RunStrategy::InProcess, tx);
    let result = rx.recv().unwrap().result;
    assert_eq!(result, TrFailedMsg(failed_msg));
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=test_should_panic_but_succeeds | COMPLEXITY=11 | LINES=38

```rust
#[test]
#[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
fn test_should_panic_but_succeeds() {
    let should_panic_variants = [ShouldPanic::Yes, ShouldPanic::YesWithMessage("error message")];

    for &should_panic in should_panic_variants.iter() {
        fn f() -> Result<(), String> {
            Ok(())
        }
        let desc = TestDescAndFn {
            desc: TestDesc {
                name: StaticTestName("whatever"),
                ignore: false,
                ignore_message: None,
                source_file: "",
                start_line: 0,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                should_panic,
                compile_fail: false,
                no_run: false,
                test_type: TestType::Unknown,
            },
            testfn: DynTestFn(Box::new(f)),
        };
        let (tx, rx) = channel();
        run_test(&TestOpts::new(), false, TestId(0), desc, RunStrategy::InProcess, tx);
        let result = rx.recv().unwrap().result;
        assert_eq!(
            result,
            TrFailedMsg("test did not panic as expected".to_string()),
            "should_panic == {:?}",
            should_panic
        );
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=report_time_test_template | COMPLEXITY=11 | LINES=30

```rust
fn report_time_test_template(report_time: bool) -> Option<TestExecTime> {
    fn f() -> Result<(), String> {
        Ok(())
    }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            ignore_message: None,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::No,
            compile_fail: false,
            no_run: false,
            test_type: TestType::Unknown,
        },
        testfn: DynTestFn(Box::new(f)),
    };
    let time_options = if report_time { Some(TestTimeOptions::default()) } else { None };

    let test_opts = TestOpts { time_options, ..TestOpts::new() };
    let (tx, rx) = channel();
    run_test(&test_opts, false, TestId(0), desc, RunStrategy::InProcess, tx);
    let exec_time = rx.recv().unwrap().exec_time;
    exec_time
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=test_should_not_report_time | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn test_should_not_report_time() {
    let exec_time = report_time_test_template(false);
    assert!(exec_time.is_none());
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=test_should_report_time | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn test_should_report_time() {
    let exec_time = report_time_test_template(true);
    assert!(exec_time.is_some());
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=time_test_failure_template | COMPLEXITY=8 | LINES=33

```rust
fn time_test_failure_template(test_type: TestType) -> TestResult {
    fn f() -> Result<(), String> {
        Ok(())
    }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            ignore_message: None,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::No,
            compile_fail: false,
            no_run: false,
            test_type,
        },
        testfn: DynTestFn(Box::new(f)),
    };
    // `Default` will initialize all the thresholds to 0 milliseconds.
    let mut time_options = TestTimeOptions::default();
    time_options.error_on_excess = true;

    let test_opts = TestOpts { time_options: Some(time_options), ..TestOpts::new() };
    let (tx, rx) = channel();
    run_test(&test_opts, false, TestId(0), desc, RunStrategy::InProcess, tx);
    let result = rx.recv().unwrap().result;

    result
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=test_error_on_exceed | COMPLEXITY=7 | LINES=15

```rust
#[test]
fn test_error_on_exceed() {
    let types = [TestType::UnitTest, TestType::IntegrationTest, TestType::DocTest];

    for test_type in types.iter() {
        let result = time_test_failure_template(*test_type);

        assert_eq!(result, TestResult::TrTimedFail);
    }

    // Check that for unknown tests thresholds aren't applied.
    let result = time_test_failure_template(TestType::Unknown);
    assert_eq!(result, TestResult::TrOk);
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=typed_test_desc | COMPLEXITY=3 | LINES=17

```rust
fn typed_test_desc(test_type: TestType) -> TestDesc {
    TestDesc {
        name: StaticTestName("whatever"),
        ignore: false,
        ignore_message: None,
        source_file: "",
        start_line: 0,
        start_col: 0,
        end_line: 0,
        end_col: 0,
        should_panic: ShouldPanic::No,
        compile_fail: false,
        no_run: false,
        test_type,
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=test_exec_time | COMPLEXITY=2 | LINES=4

```rust
fn test_exec_time(millis: u64) -> TestExecTime {
    TestExecTime(Duration::from_millis(millis))
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=test_time_options_threshold | COMPLEXITY=9 | LINES=34

```rust
#[test]
fn test_time_options_threshold() {
    let unit = TimeThreshold::new(Duration::from_millis(50), Duration::from_millis(100));
    let integration = TimeThreshold::new(Duration::from_millis(500), Duration::from_millis(1000));
    let doc = TimeThreshold::new(Duration::from_millis(5000), Duration::from_millis(10000));

    let options = TestTimeOptions {
        error_on_excess: false,
        unit_threshold: unit.clone(),
        integration_threshold: integration.clone(),
        doctest_threshold: doc.clone(),
    };

    let test_vector = [
        (TestType::UnitTest, unit.warn.as_millis() - 1, false, false),
        (TestType::UnitTest, unit.warn.as_millis(), true, false),
        (TestType::UnitTest, unit.critical.as_millis(), true, true),
        (TestType::IntegrationTest, integration.warn.as_millis() - 1, false, false),
        (TestType::IntegrationTest, integration.warn.as_millis(), true, false),
        (TestType::IntegrationTest, integration.critical.as_millis(), true, true),
        (TestType::DocTest, doc.warn.as_millis() - 1, false, false),
        (TestType::DocTest, doc.warn.as_millis(), true, false),
        (TestType::DocTest, doc.critical.as_millis(), true, true),
    ];

    for (test_type, time, expected_warn, expected_critical) in test_vector.iter() {
        let test_desc = typed_test_desc(*test_type);
        let exec_time = test_exec_time(*time as u64);

        assert_eq!(options.is_warn(&test_desc, &exec_time), *expected_warn);
        assert_eq!(options.is_critical(&test_desc, &exec_time), *expected_critical);
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=parse_ignored_flag | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn parse_ignored_flag() {
    let args = vec!["progname".to_string(), "filter".to_string(), "--ignored".to_string()];
    let opts = parse_opts(&args).unwrap().unwrap();
    assert_eq!(opts.run_ignored, RunIgnored::Only);
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=parse_show_output_flag | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn parse_show_output_flag() {
    let args = vec!["progname".to_string(), "filter".to_string(), "--show-output".to_string()];
    let opts = parse_opts(&args).unwrap().unwrap();
    assert!(opts.options.display_output);
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=parse_include_ignored_flag | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn parse_include_ignored_flag() {
    let args = vec!["progname".to_string(), "filter".to_string(), "--include-ignored".to_string()];
    let opts = parse_opts(&args).unwrap().unwrap();
    assert_eq!(opts.run_ignored, RunIgnored::Yes);
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=filter_for_ignored_option | COMPLEXITY=3 | LINES=17

```rust
#[test]
fn filter_for_ignored_option() {
    // When we run ignored tests the test filter should filter out all the
    // unignored tests and flip the ignore flag on the rest to false

    let mut opts = TestOpts::new();
    opts.run_tests = true;
    opts.run_ignored = RunIgnored::Only;

    let tests = one_ignored_one_unignored_test();
    let filtered = filter_tests(&opts, tests);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].desc.name.to_string(), "1");
    assert!(!filtered[0].desc.ignore);
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=run_include_ignored_option | COMPLEXITY=2 | LINES=17

```rust
#[test]
fn run_include_ignored_option() {
    // When we "--include-ignored" tests, the ignore flag should be set to false on
    // all tests and no test filtered out

    let mut opts = TestOpts::new();
    opts.run_tests = true;
    opts.run_ignored = RunIgnored::Yes;

    let tests = one_ignored_one_unignored_test();
    let filtered = filter_tests(&opts, tests);

    assert_eq!(filtered.len(), 2);
    assert!(!filtered[0].desc.ignore);
    assert!(!filtered[1].desc.ignore);
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=exclude_should_panic_option | COMPLEXITY=5 | LINES=31

```rust
#[test]
fn exclude_should_panic_option() {
    let mut opts = TestOpts::new();
    opts.run_tests = true;
    opts.exclude_should_panic = true;

    let mut tests = one_ignored_one_unignored_test();
    tests.push(TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("3"),
            ignore: false,
            ignore_message: None,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::Yes,
            compile_fail: false,
            no_run: false,
            test_type: TestType::Unknown,
        },
        testfn: DynTestFn(Box::new(move || Ok(()))),
    });

    let filtered = filter_tests(&opts, tests);

    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|test| test.desc.should_panic == ShouldPanic::No));
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=exact_filter_match | COMPLEXITY=20 | LINES=82

```rust
#[test]
fn exact_filter_match() {
    fn tests() -> Vec<TestDescAndFn> {
        ["base", "base::test", "base::test1", "base::test2"]
            .into_iter()
            .map(|name| TestDescAndFn {
                desc: TestDesc {
                    name: StaticTestName(name),
                    ignore: false,
                    ignore_message: None,
                    source_file: "",
                    start_line: 0,
                    start_col: 0,
                    end_line: 0,
                    end_col: 0,
                    should_panic: ShouldPanic::No,
                    compile_fail: false,
                    no_run: false,
                    test_type: TestType::Unknown,
                },
                testfn: DynTestFn(Box::new(move || Ok(()))),
            })
            .collect()
    }

    let substr =
        filter_tests(&TestOpts { filters: vec!["base".into()], ..TestOpts::new() }, tests());
    assert_eq!(substr.len(), 4);

    let substr =
        filter_tests(&TestOpts { filters: vec!["bas".into()], ..TestOpts::new() }, tests());
    assert_eq!(substr.len(), 4);

    let substr =
        filter_tests(&TestOpts { filters: vec!["::test".into()], ..TestOpts::new() }, tests());
    assert_eq!(substr.len(), 3);

    let substr =
        filter_tests(&TestOpts { filters: vec!["base::test".into()], ..TestOpts::new() }, tests());
    assert_eq!(substr.len(), 3);

    let substr = filter_tests(
        &TestOpts { filters: vec!["test1".into(), "test2".into()], ..TestOpts::new() },
        tests(),
    );
    assert_eq!(substr.len(), 2);

    let exact = filter_tests(
        &TestOpts { filters: vec!["base".into()], filter_exact: true, ..TestOpts::new() },
        tests(),
    );
    assert_eq!(exact.len(), 1);

    let exact = filter_tests(
        &TestOpts { filters: vec!["bas".into()], filter_exact: true, ..TestOpts::new() },
        tests(),
    );
    assert_eq!(exact.len(), 0);

    let exact = filter_tests(
        &TestOpts { filters: vec!["::test".into()], filter_exact: true, ..TestOpts::new() },
        tests(),
    );
    assert_eq!(exact.len(), 0);

    let exact = filter_tests(
        &TestOpts { filters: vec!["base::test".into()], filter_exact: true, ..TestOpts::new() },
        tests(),
    );
    assert_eq!(exact.len(), 1);

    let exact = filter_tests(
        &TestOpts {
            filters: vec!["base".into(), "base::test".into()],
            filter_exact: true,
            ..TestOpts::new()
        },
        tests(),
    );
    assert_eq!(exact.len(), 2);
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=sample_tests | COMPLEXITY=10 | LINES=41

```rust
fn sample_tests() -> Vec<TestDescAndFn> {
    let names = vec![
        "sha1::test".to_string(),
        "isize::test_to_str".to_string(),
        "isize::test_pow".to_string(),
        "test::do_not_run_ignored_tests".to_string(),
        "test::ignored_tests_result_in_ignored".to_string(),
        "test::first_free_arg_should_be_a_filter".to_string(),
        "test::parse_ignored_flag".to_string(),
        "test::parse_include_ignored_flag".to_string(),
        "test::filter_for_ignored_option".to_string(),
        "test::run_include_ignored_option".to_string(),
        "test::sort_tests".to_string(),
    ];
    fn testfn() -> Result<(), String> {
        Ok(())
    }
    let mut tests = Vec::new();
    for name in &names {
        let test = TestDescAndFn {
            desc: TestDesc {
                name: DynTestName((*name).clone()),
                ignore: false,
                ignore_message: None,
                source_file: "",
                start_line: 0,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                should_panic: ShouldPanic::No,
                compile_fail: false,
                no_run: false,
                test_type: TestType::Unknown,
            },
            testfn: DynTestFn(Box::new(testfn)),
        };
        tests.push(test);
    }
    tests
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=shuffle_tests | COMPLEXITY=3 | LINES=19

```rust
#[test]
fn shuffle_tests() {
    let mut opts = TestOpts::new();
    opts.shuffle = true;

    let shuffle_seed = get_shuffle_seed(&opts).unwrap();

    let left =
        sample_tests().into_iter().enumerate().map(|(i, e)| (TestId(i), e)).collect::<Vec<_>>();
    let mut right =
        sample_tests().into_iter().enumerate().map(|(i, e)| (TestId(i), e)).collect::<Vec<_>>();

    assert!(left.iter().zip(&right).all(|(a, b)| a.1.desc.name == b.1.desc.name));

    helpers::shuffle::shuffle_tests(shuffle_seed, right.as_mut_slice());

    assert!(left.iter().zip(right).any(|(a, b)| a.1.desc.name != b.1.desc.name));
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=shuffle_tests_with_seed | COMPLEXITY=3 | LINES=18

```rust
#[test]
fn shuffle_tests_with_seed() {
    let mut opts = TestOpts::new();
    opts.shuffle = true;

    let shuffle_seed = get_shuffle_seed(&opts).unwrap();

    let mut left =
        sample_tests().into_iter().enumerate().map(|(i, e)| (TestId(i), e)).collect::<Vec<_>>();
    let mut right =
        sample_tests().into_iter().enumerate().map(|(i, e)| (TestId(i), e)).collect::<Vec<_>>();

    helpers::shuffle::shuffle_tests(shuffle_seed, left.as_mut_slice());
    helpers::shuffle::shuffle_tests(shuffle_seed, right.as_mut_slice());

    assert!(left.iter().zip(right).all(|(a, b)| a.1.desc.name == b.1.desc.name));
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=order_depends_on_more_than_seed | COMPLEXITY=3 | LINES=28

```rust
#[test]
fn order_depends_on_more_than_seed() {
    let mut opts = TestOpts::new();
    opts.shuffle = true;

    let shuffle_seed = get_shuffle_seed(&opts).unwrap();

    let mut left_tests = sample_tests();
    let mut right_tests = sample_tests();

    left_tests.pop();
    right_tests.remove(0);

    let mut left =
        left_tests.into_iter().enumerate().map(|(i, e)| (TestId(i), e)).collect::<Vec<_>>();
    let mut right =
        right_tests.into_iter().enumerate().map(|(i, e)| (TestId(i), e)).collect::<Vec<_>>();

    assert_eq!(left.len(), right.len());

    assert!(left.iter().zip(&right).all(|(a, b)| a.0 == b.0));

    helpers::shuffle::shuffle_tests(shuffle_seed, left.as_mut_slice());
    helpers::shuffle::shuffle_tests(shuffle_seed, right.as_mut_slice());

    assert!(left.iter().zip(right).any(|(a, b)| a.0 != b.0));
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=test_metricmap_compare | COMPLEXITY=3 | LINES=23

```rust
#[test]
fn test_metricmap_compare() {
    let mut m1 = MetricMap::new();
    let mut m2 = MetricMap::new();
    m1.insert_metric("in-both-noise", 1000.0, 200.0);
    m2.insert_metric("in-both-noise", 1100.0, 200.0);

    m1.insert_metric("in-first-noise", 1000.0, 2.0);
    m2.insert_metric("in-second-noise", 1000.0, 2.0);

    m1.insert_metric("in-both-want-downwards-but-regressed", 1000.0, 10.0);
    m2.insert_metric("in-both-want-downwards-but-regressed", 2000.0, 10.0);

    m1.insert_metric("in-both-want-downwards-and-improved", 2000.0, 10.0);
    m2.insert_metric("in-both-want-downwards-and-improved", 1000.0, 10.0);

    m1.insert_metric("in-both-want-upwards-but-regressed", 2000.0, -10.0);
    m2.insert_metric("in-both-want-upwards-but-regressed", 1000.0, -10.0);

    m1.insert_metric("in-both-want-upwards-and-improved", 1000.0, -10.0);
    m2.insert_metric("in-both-want-upwards-and-improved", 2000.0, -10.0);
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=test_bench_once_no_iter | COMPLEXITY=3 | LINES=8

```rust
#[test]
fn test_bench_once_no_iter() {
    fn f(_: &mut Bencher) -> Result<(), String> {
        Ok(())
    }
    bench::run_once(f).unwrap();
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=test_bench_once_iter | COMPLEXITY=4 | LINES=9

```rust
#[test]
fn test_bench_once_iter() {
    fn f(b: &mut Bencher) -> Result<(), String> {
        b.iter(|| {});
        Ok(())
    }
    bench::run_once(f).unwrap();
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=test_bench_no_iter | COMPLEXITY=5 | LINES=27

```rust
#[test]
fn test_bench_no_iter() {
    fn f(_: &mut Bencher) -> Result<(), String> {
        Ok(())
    }

    let (tx, rx) = channel();

    let desc = TestDesc {
        name: StaticTestName("f"),
        ignore: false,
        ignore_message: None,
        source_file: "",
        start_line: 0,
        start_col: 0,
        end_line: 0,
        end_col: 0,
        should_panic: ShouldPanic::No,
        compile_fail: false,
        no_run: false,
        test_type: TestType::Unknown,
    };

    crate::bench::benchmark(TestId(0), desc, tx, true, f);
    rx.recv().unwrap();
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=test_bench_iter | COMPLEXITY=6 | LINES=28

```rust
#[test]
fn test_bench_iter() {
    fn f(b: &mut Bencher) -> Result<(), String> {
        b.iter(|| {});
        Ok(())
    }

    let (tx, rx) = channel();

    let desc = TestDesc {
        name: StaticTestName("f"),
        ignore: false,
        ignore_message: None,
        source_file: "",
        start_line: 0,
        start_col: 0,
        end_line: 0,
        end_col: 0,
        should_panic: ShouldPanic::No,
        compile_fail: false,
        no_run: false,
        test_type: TestType::Unknown,
    };

    crate::bench::benchmark(TestId(0), desc, tx, true, f);
    rx.recv().unwrap();
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=should_sort_failures_before_printing_them | COMPLEXITY=12 | LINES=62

```rust
#[test]
fn should_sort_failures_before_printing_them() {
    let test_a = TestDesc {
        name: StaticTestName("a"),
        ignore: false,
        ignore_message: None,
        source_file: "",
        start_line: 0,
        start_col: 0,
        end_line: 0,
        end_col: 0,
        should_panic: ShouldPanic::No,
        compile_fail: false,
        no_run: false,
        test_type: TestType::Unknown,
    };

    let test_b = TestDesc {
        name: StaticTestName("b"),
        ignore: false,
        ignore_message: None,
        source_file: "",
        start_line: 0,
        start_col: 0,
        end_line: 0,
        end_col: 0,
        should_panic: ShouldPanic::No,
        compile_fail: false,
        no_run: false,
        test_type: TestType::Unknown,
    };

    let mut out = PrettyFormatter::new(OutputLocation::Raw(Vec::new()), false, 10, false, None);

    let st = console::ConsoleTestState {
        log_out: None,
        total: 0,
        passed: 0,
        failed: 0,
        ignored: 0,
        filtered_out: 0,
        measured: 0,
        exec_time: None,
        metrics: MetricMap::new(),
        failures: vec![(test_b, Vec::new()), (test_a, Vec::new())],
        options: Options::new(),
        not_failures: Vec::new(),
        ignores: Vec::new(),
        time_failures: Vec::new(),
    };

    out.write_failures(&st).unwrap();
    let s = match out.output_location() {
        &OutputLocation::Raw(ref m) => String::from_utf8_lossy(&m[..]),
        &OutputLocation::Pretty(_) => unreachable!(),
    };

    let apos = s.find("a").unwrap();
    let bpos = s.find("b").unwrap();
    assert!(apos < bpos);
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=test_dyn_bench_returning_err_fails_when_run_as_test | COMPLEXITY=12 | LINES=35

```rust
#[test]
#[cfg(not(target_os = "emscripten"))]
fn test_dyn_bench_returning_err_fails_when_run_as_test() {
    fn f(_: &mut Bencher) -> Result<(), String> {
        Result::Err("An error".into())
    }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            ignore_message: None,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::No,
            compile_fail: false,
            no_run: false,
            test_type: TestType::Unknown,
        },
        testfn: DynBenchFn(Box::new(f)),
    };
    let (tx, rx) = channel();
    let notify = move |event: TestEvent| {
        if let TestEvent::TeResult(result) = event {
            tx.send(result).unwrap();
        }
        Ok(())
    };
    run_tests(&TestOpts { run_tests: true, ..TestOpts::new() }, vec![desc], notify).unwrap();
    let result = rx.recv().unwrap().result;
    assert_eq!(result, TrFailed);
}
```

---
*Generated by AST tracing system*
