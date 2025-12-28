macro_rules! deps {
    () => {
        Float!();
        Int!();
    };
}

macro_rules! float_impl {
    () => {
        deps!();
        macro_rules ! float_impl { ($ ty : ident , $ ity : ident , $ sity : ident , $ bits : expr , $ significand_bits : expr , $ from_bits : path , $ to_bits : path , $ fma_fn : ident , $ fma_intrinsic : ident) => { impl Float for $ ty { type Int = $ ity ; type SignedInt = $ sity ; const ZERO : Self = 0.0 ; const NEG_ZERO : Self = - 0.0 ; const ONE : Self = 1.0 ; const NEG_ONE : Self = - 1.0 ; const INFINITY : Self = Self :: INFINITY ; const NEG_INFINITY : Self = Self :: NEG_INFINITY ; const NAN : Self = Self :: NAN ; const NEG_NAN : Self = $ from_bits ($ to_bits (Self :: NAN) | Self :: SIGN_MASK) ; const MAX : Self = - Self :: MIN ; const MIN : Self = $ from_bits (Self :: Int :: MAX & ! (1 << Self :: SIG_BITS)) ; const EPSILON : Self = <$ ty >:: EPSILON ; const MIN_POSITIVE_NORMAL : Self = $ from_bits (1 << Self :: SIG_BITS) ; const PI : Self = core ::$ ty :: consts :: PI ; const NEG_PI : Self = - Self :: PI ; const FRAC_PI_2 : Self = core ::$ ty :: consts :: FRAC_PI_2 ; const BITS : u32 = $ bits ; const SIG_BITS : u32 = $ significand_bits ; const SIGN_MASK : Self :: Int = 1 << (Self :: BITS - 1) ; const SIG_MASK : Self :: Int = (1 << Self :: SIG_BITS) - 1 ; const EXP_MASK : Self :: Int = ! (Self :: SIGN_MASK | Self :: SIG_MASK) ; const IMPLICIT_BIT : Self :: Int = 1 << Self :: SIG_BITS ; fn to_bits (self) -> Self :: Int { self . to_bits () } fn is_nan (self) -> bool { self . is_nan () } fn is_infinite (self) -> bool { self . is_infinite () } fn is_sign_negative (self) -> bool { self . is_sign_negative () } fn from_bits (a : Self :: Int) -> Self { Self :: from_bits (a) } fn abs (self) -> Self { cfg_if ! { if # [cfg (intrinsics_enabled)] { self . abs () } else { super :: super :: generic :: fabs (self) } } } fn copysign (self , other : Self) -> Self { cfg_if ! { if # [cfg (intrinsics_enabled)] { self . copysign (other) } else { super :: super :: generic :: copysign (self , other) } } } fn fma (self , y : Self , z : Self) -> Self { cfg_if ! { if # [cfg (intrinsics_enabled)] { # [allow (unused_unsafe)] unsafe { core :: intrinsics ::$ fma_intrinsic (self , y , z) } } else { super :: super ::$ fma_fn (self , y , z) } } } fn normalize (significand : Self :: Int) -> (i32 , Self :: Int) { let shift = significand . leading_zeros () . wrapping_sub (Self :: EXP_BITS) ; (1i32 . wrapping_sub (shift as i32) , significand << shift as Self :: Int ,) } } } ; }
    };
}

float_impl!()