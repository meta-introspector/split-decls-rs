macro_rules! run_and_expect_no_errors {
    () => {
        # [track_caller] fn run_and_expect_no_errors (path : & str) { run_and_expect_no_errors_with_edition (path , Edition :: CURRENT) }
    };
}

run_and_expect_no_errors!();