macro_rules! deps {
    () => {
        Word!();
    };
}

macro_rules! overflowing_add {
    () => {
        deps!();
        # [doc = " Computes `lhs + rhs`, returning the result along with the carry (0 or 1)."] # [inline (always)] pub (crate) const fn overflowing_add (lhs : Word , rhs : Word) -> (Word , Word) { let (res , carry) = lhs . overflowing_add (rhs) ; (res , carry as Word) }
    };
}

overflowing_add!();