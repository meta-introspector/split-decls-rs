macro_rules! deps {
    () => {
        DemangleAsInner!();
        DemangleWrite!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for FunctionArgListAndReturnType where W : 'subs + DemangleWrite { }
    };
}

impl_59!();