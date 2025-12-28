macro_rules! deps {
    () => {
        GenericArrayImplOdd!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T : ConstDefault , U : ConstDefault > ConstDefault for GenericArrayImplOdd < T , U > { const DEFAULT : Self = Self { parents : [U :: DEFAULT ; 2] , data : T :: DEFAULT , } ; }
    };
}

impl_71!()