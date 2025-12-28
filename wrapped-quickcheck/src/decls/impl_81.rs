macro_rules! deps {
    () => {
        TestResult!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl From < bool > for TestResult { # [doc = " A shorter way of producing a `TestResult` from a `bool`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use quickcheck::TestResult;"] # [doc = " let result: TestResult = (2 > 1).into();"] # [doc = " assert_eq!(result, TestResult::passed());"] # [doc = " ```"] fn from (b : bool) -> TestResult { TestResult :: from_bool (b) } }
    };
}

impl_81!()