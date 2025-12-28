macro_rules! deps {
    () => {
        WideWord!();
        Word!();
    };
}

macro_rules! widening_mul {
    () => {
        deps!();
        # [doc = " Computes `lhs * rhs`, returning the low and the high words of the result."] # [inline (always)] pub (crate) const fn widening_mul (lhs : Word , rhs : Word) -> (Word , Word) { let a = lhs as WideWord ; let b = rhs as WideWord ; let ret = a * b ; (ret as Word , (ret >> Word :: BITS) as Word) }
    };
}

widening_mul!();