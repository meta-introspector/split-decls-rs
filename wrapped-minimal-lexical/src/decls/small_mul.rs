macro_rules! deps {
    () => {
        Limb!();
        VecType!();
    };
}

macro_rules! small_mul {
    () => {
        deps!();
        # [doc = " Multiply bigint by small integer."] # [inline] pub fn small_mul (x : & mut VecType , y : Limb) -> Option < () > { let mut carry = 0 ; for xi in x . iter_mut () { let result = scalar_mul (* xi , y , carry) ; * xi = result . 0 ; carry = result . 1 ; } if carry != 0 { x . try_push (carry) ? ; } Some (()) }
    };
}

small_mul!();