macro_rules! deps {
    () => {
        Gen!();
        Testable!();
        TestResult!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl Testable for TestResult { fn result (& self , _ : & mut Gen) -> TestResult { self . clone () } }
    };
}

impl_85!();