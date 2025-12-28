macro_rules! deps {
    () => {
        Number!();
    };
}

macro_rules! Integer {
    () => {
        deps!();
        # [doc = " Defines a trait that supports integral operations."] pub trait Integer : Number + ops :: BitAnd < Output = Self > + ops :: Shr < i32 , Output = Self > { const ZERO : Self ; }
    };
}

Integer!()