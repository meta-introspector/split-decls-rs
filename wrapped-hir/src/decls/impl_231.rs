macro_rules! deps {
    () => {
        MethodCandidateCallback!();
        Function!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < F > MethodCandidateCallback for F where F : FnMut (Function) -> ControlFlow < () > , { fn on_inherent_method (& mut self , f : Function) -> ControlFlow < () > { self (f) } fn on_trait_method (& mut self , f : Function) -> ControlFlow < () > { self (f) } }
    };
}

impl_231!()