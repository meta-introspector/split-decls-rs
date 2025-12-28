macro_rules! deps {
    () => {
        MZError!();
        TDEFLFlush!();
        Result!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl TDEFLFlush { pub const fn new (flush : i32) -> Result < Self , MZError > { match flush { 0 => Ok (TDEFLFlush :: None) , 1 => Ok (TDEFLFlush :: Partial) , 2 => Ok (TDEFLFlush :: Sync) , 3 => Ok (TDEFLFlush :: Full) , 4 => Ok (TDEFLFlush :: Finish) , _ => Err (MZError :: Param) , } } }
    };
}

impl_34!();