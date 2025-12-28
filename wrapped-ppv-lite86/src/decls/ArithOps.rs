macro_rules! deps {
    () => {
        BSwap!();
    };
}

macro_rules! ArithOps {
    () => {
        deps!();
        # [doc = " Ops that depend on word size"] pub trait ArithOps : Add < Output = Self > + AddAssign + Sized + Copy + Clone + BSwap { }
    };
}

ArithOps!();