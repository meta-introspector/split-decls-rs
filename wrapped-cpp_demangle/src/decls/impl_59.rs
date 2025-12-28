macro_rules! deps {
    () => {
        DemangleWrite!();
        DemangleAsInner!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for FunctionArgListAndReturnType where W : 'subs + DemangleWrite { }
    };
}

impl_59!()