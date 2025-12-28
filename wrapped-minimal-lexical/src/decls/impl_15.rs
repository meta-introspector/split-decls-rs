macro_rules! deps {
    () => {
        Bigint!();
        VecType!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [allow (clippy :: new_without_default)] impl Bigint { # [doc = " Construct a bigint representing 0."] # [inline (always)] pub fn new () -> Self { Self { data : VecType :: new () , } } # [doc = " Construct a bigint from an integer."] # [inline (always)] pub fn from_u64 (value : u64) -> Self { Self { data : VecType :: from_u64 (value) , } } # [inline (always)] pub fn hi64 (& self) -> (u64 , bool) { self . data . hi64 () } # [doc = " Multiply and assign as if by exponentiation by a power."] # [inline] pub fn pow (& mut self , base : u32 , exp : u32) -> Option < () > { debug_assert ! (base == 2 || base == 5 || base == 10) ; if base % 5 == 0 { pow (& mut self . data , exp) ? ; } if base % 2 == 0 { shl (& mut self . data , exp as usize) ? ; } Some (()) } # [doc = " Calculate the bit-length of the big-integer."] # [inline] pub fn bit_length (& self) -> u32 { bit_length (& self . data) } }
    };
}

impl_15!()