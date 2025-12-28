macro_rules! deps {
    () => {
        Result!();
        MZFlush!();
        MZError!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl MZFlush { # [doc = " Create an MZFlush value from an integer value."] # [doc = ""] # [doc = " Returns `MZError::Param` on invalid values."] pub fn new (flush : i32) -> Result < Self , MZError > { match flush { 0 => Ok (MZFlush :: None) , 1 | 2 => Ok (MZFlush :: Sync) , 3 => Ok (MZFlush :: Full) , 4 => Ok (MZFlush :: Finish) , _ => Err (MZError :: Param) , } } }
    };
}

impl_229!()