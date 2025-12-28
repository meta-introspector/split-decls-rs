macro_rules! deps {
    () => {
        Testable!();
        Gen!();
        TestResult!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Testable for () { fn result (& self , _ : & mut Gen) -> TestResult { TestResult :: passed () } }
    };
}

impl_84!();