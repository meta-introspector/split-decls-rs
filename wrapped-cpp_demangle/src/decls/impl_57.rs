macro_rules! deps {
    () => {
        DemangleWrite!();
        DemangleAsInner!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for FunctionArgList where W : 'subs + DemangleWrite { }
    };
}

impl_57!()