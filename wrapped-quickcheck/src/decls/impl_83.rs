macro_rules! deps {
    () => {
        TestResult!();
        Testable!();
        Gen!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl Testable for bool { fn result (& self , _ : & mut Gen) -> TestResult { TestResult :: from_bool (* self) } }
    };
}

impl_83!()