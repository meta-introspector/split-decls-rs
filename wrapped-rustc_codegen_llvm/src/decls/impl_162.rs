macro_rules! deps {
    () => {
        GenericBuilder!();
        SCx!();
        GenericCx!();
        Funclet!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < 'll , CX : Borrow < SCx < 'll > > > BackendTypes for GenericBuilder < '_ , 'll , CX > { type Value = < GenericCx < 'll , CX > as BackendTypes > :: Value ; type Metadata = < GenericCx < 'll , CX > as BackendTypes > :: Metadata ; type Function = < GenericCx < 'll , CX > as BackendTypes > :: Function ; type BasicBlock = < GenericCx < 'll , CX > as BackendTypes > :: BasicBlock ; type Type = < GenericCx < 'll , CX > as BackendTypes > :: Type ; type Funclet = < GenericCx < 'll , CX > as BackendTypes > :: Funclet ; type DIScope = < GenericCx < 'll , CX > as BackendTypes > :: DIScope ; type DILocation = < GenericCx < 'll , CX > as BackendTypes > :: DILocation ; type DIVariable = < GenericCx < 'll , CX > as BackendTypes > :: DIVariable ; }
    };
}

impl_162!();