macro_rules! deps {
    () => {
        HirDisplayError!();
        DisplaySourceCodeError!();
        HirDisplay!();
        HirDisplayWrapper!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        impl < 'db , T > fmt :: Display for HirDisplayWrapper < '_ , 'db , T > where T : HirDisplay < 'db > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . write_to (f) { Ok (()) => Ok (()) , Err (HirDisplayError :: FmtError) => Err (fmt :: Error) , Err (HirDisplayError :: DisplaySourceCodeError (_)) => { panic ! ("HirDisplay::hir_fmt failed with DisplaySourceCodeError when calling Display::fmt!") } } } }
    };
}

impl_494!()