macro_rules! deps {
    () => {
        Word!();
        WideWord!();
    };
}

macro_rules! carrying_mul_add {
    () => {
        deps!();
        # [doc = " Computes `(lhs * rhs) + addend + carry`, returning the result along with the new carry."] # [inline (always)] pub (crate) const fn carrying_mul_add (lhs : Word , rhs : Word , addend : Word , carry : Word ,) -> (Word , Word) { let lhs = lhs as WideWord ; let rhs = rhs as WideWord ; let addend = addend as WideWord ; let carry = carry as WideWord ; let ret = ((lhs * rhs) + addend) + carry ; (ret as Word , (ret >> Word :: BITS) as Word) }
    };
}

carrying_mul_add!();