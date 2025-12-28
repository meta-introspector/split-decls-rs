macro_rules! deps {
    () => {
        Funclet!();
        SCx!();
        GenericCx!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < 'll , CX : Borrow < SCx < 'll > > > BackendTypes for GenericCx < 'll , CX > { type Value = & 'll Value ; type Metadata = & 'll Metadata ; type Function = & 'll Value ; type BasicBlock = & 'll BasicBlock ; type Type = & 'll Type ; type Funclet = Funclet < 'll > ; type DIScope = & 'll llvm :: debuginfo :: DIScope ; type DILocation = & 'll llvm :: debuginfo :: DILocation ; type DIVariable = & 'll llvm :: debuginfo :: DIVariable ; }
    };
}

impl_186!()