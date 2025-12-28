macro_rules! deps {
    () => {
        GenericArrayImplEven!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < T , U : ConstDefault > ConstDefault for GenericArrayImplEven < T , U > { const DEFAULT : Self = Self { parents : [U :: DEFAULT ; 2] , _marker : core :: marker :: PhantomData , } ; }
    };
}

impl_70!();