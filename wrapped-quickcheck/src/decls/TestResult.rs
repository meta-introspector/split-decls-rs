macro_rules! deps {
    () => {
        Status!();
    };
}

macro_rules! TestResult {
    () => {
        deps!();
        # [doc = " Describes the status of a single instance of a test."] # [doc = ""] # [doc = " All testable things must be capable of producing a `TestResult`."] # [derive (Clone , Debug , PartialEq)] pub struct TestResult { status : Status , arguments : Option < Vec < String > > , err : Option < String > , }
    };
}

TestResult!()