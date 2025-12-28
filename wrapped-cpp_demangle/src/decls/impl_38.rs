macro_rules! deps {
    () => {
        DemangleWrite!();
        AutoParseDemangle!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < 'a , 'b , W : 'a + DemangleWrite > Drop for AutoParseDemangle < 'a , 'b , W > { # [inline] fn drop (& mut self) { self . 0 . exit_recursion () ; } }
    };
}

impl_38!();