macro_rules! deps {
    () => {
        DotDotPos!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl DotDotPos { # [doc = " Panics if n >= u32::MAX."] pub fn new (n : Option < usize >) -> Self { match n { Some (n) => { assert ! (n < u32 :: MAX as usize) ; Self (n as u32) } None => Self (u32 :: MAX) , } } pub fn as_opt_usize (& self) -> Option < usize > { if self . 0 == u32 :: MAX { None } else { Some (self . 0 as usize) } } }
    };
}

impl_187!()