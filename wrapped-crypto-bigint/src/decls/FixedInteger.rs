macro_rules! deps {
    () => {
        Constants!();
        Integer!();
        Bounded!();
    };
}

macro_rules! FixedInteger {
    () => {
        deps!();
        # [doc = " Fixed-width [`Integer`]s."] pub trait FixedInteger : Bounded + ConditionallySelectable + Constants + Copy + Integer { # [doc = " The number of limbs used on this platform."] const LIMBS : usize ; }
    };
}

FixedInteger!()