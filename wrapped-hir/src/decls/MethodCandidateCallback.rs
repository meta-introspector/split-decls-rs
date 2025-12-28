macro_rules! deps {
    () => {
        Function!();
    };
}

macro_rules! MethodCandidateCallback {
    () => {
        deps!();
        pub trait MethodCandidateCallback { fn on_inherent_method (& mut self , f : Function) -> ControlFlow < () > ; fn on_trait_method (& mut self , f : Function) -> ControlFlow < () > ; }
    };
}

MethodCandidateCallback!();