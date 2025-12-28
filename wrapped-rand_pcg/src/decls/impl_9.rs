macro_rules! deps {
    () => {
        Mcg128Xsl64!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Mcg128Xsl64 { # [doc = " Multi-step advance functions (jump-ahead, jump-back)"] # [doc = ""] # [doc = " The method used here is based on Brown, \"Random Number Generation"] # [doc = " with Arbitrary Stride,\", Transactions of the American Nuclear"] # [doc = " Society (Nov. 1994).  The algorithm is very similar to fast"] # [doc = " exponentiation."] # [doc = ""] # [doc = " Even though delta is an unsigned integer, we can pass a"] # [doc = " signed integer to go backwards, it just goes \"the long way round\"."] # [doc = ""] # [doc = " Using this function is equivalent to calling `next_64()` `delta`"] # [doc = " number of times."] # [inline] pub fn advance (& mut self , delta : u128) { let mut acc_mult : u128 = 1 ; let mut acc_plus : u128 = 0 ; let mut cur_mult = MULTIPLIER ; let mut cur_plus : u128 = 0 ; let mut mdelta = delta ; while mdelta > 0 { if (mdelta & 1) != 0 { acc_mult = acc_mult . wrapping_mul (cur_mult) ; acc_plus = acc_plus . wrapping_mul (cur_mult) . wrapping_add (cur_plus) ; } cur_plus = cur_mult . wrapping_add (1) . wrapping_mul (cur_plus) ; cur_mult = cur_mult . wrapping_mul (cur_mult) ; mdelta /= 2 ; } self . state = acc_mult . wrapping_mul (self . state) . wrapping_add (acc_plus) ; } # [doc = " Construct an instance compatible with PCG seed."] # [doc = ""] # [doc = " Note that PCG specifies a default value for the parameter:"] # [doc = ""] # [doc = " - `state = 0xcafef00dd15ea5e5`"] pub fn new (state : u128) -> Self { Mcg128Xsl64 { state : state | 1 } } }
    };
}

impl_9!();