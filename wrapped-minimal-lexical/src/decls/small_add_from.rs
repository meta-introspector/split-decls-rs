macro_rules! deps {
    () => {
        VecType!();
        Limb!();
    };
}

macro_rules! small_add_from {
    () => {
        deps!();
        # [doc = " Add small integer to bigint starting from offset."] # [inline] pub fn small_add_from (x : & mut VecType , y : Limb , start : usize) -> Option < () > { let mut index = start ; let mut carry = y ; while carry != 0 && index < x . len () { let result = scalar_add (x [index] , carry) ; x [index] = result . 0 ; carry = result . 1 as Limb ; index += 1 ; } if carry != 0 { x . try_push (carry) ? ; } Some (()) }
    };
}

small_add_from!()