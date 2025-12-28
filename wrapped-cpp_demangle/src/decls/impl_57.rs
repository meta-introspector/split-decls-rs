macro_rules! deps {
    () => {
        DemangleAsInner!();
        DemangleWrite!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for FunctionArgList where W : 'subs + DemangleWrite { }
    };
}

impl_57!();