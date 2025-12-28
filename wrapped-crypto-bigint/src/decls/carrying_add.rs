macro_rules! deps {
    () => {
        Word!();
        WideWord!();
    };
}

macro_rules! carrying_add {
    () => {
        deps!();
        # [doc = " Computes `lhs + rhs + carry`, returning the result along with the new carry (0, 1, or 2)."] # [inline (always)] pub (crate) const fn carrying_add (lhs : Word , rhs : Word , carry : Word) -> (Word , Word) { let a = lhs as WideWord ; let b = rhs as WideWord ; let carry = carry as WideWord ; let ret = a + b + carry ; (ret as Word , (ret >> Word :: BITS) as Word) }
    };
}

carrying_add!();