macro_rules! deps {
    () => {
        Testable!();
        Gen!();
        TestResult!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < A , E > Testable for Result < A , E > where A : Testable , E : Debug + 'static , { fn result (& self , g : & mut Gen) -> TestResult { match * self { Ok (ref r) => r . result (g) , Err (ref err) => TestResult :: error (format ! ("{err:?}")) , } } }
    };
}

impl_86!();