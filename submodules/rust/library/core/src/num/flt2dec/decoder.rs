mkuse!{use crate :: num :: FpCategory ;}
mkuse!{use crate :: num :: dec2flt :: float :: RawFloat ;}
mkitem!{mkstruct!{# [doc = " Decoded unsigned finite value, such that:"] # [doc = ""] # [doc = " - The original value equals to `mant * 2^exp`."] # [doc = ""] # [doc = " - Any number from `(mant - minus) * 2^exp` to `(mant + plus) * 2^exp` will"] # [doc = "   round to the original value. The range is inclusive only when"] # [doc = "   `inclusive` is `true`."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct Decoded { # [doc = " The scaled mantissa."] pub mant : u64 , # [doc = " The lower error range."] pub minus : u64 , # [doc = " The upper error range."] pub plus : u64 , # [doc = " The shared exponent in base 2."] pub exp : i16 , # [doc = " True when the error range is inclusive."] # [doc = ""] # [doc = " In IEEE 754, this is true when the original mantissa was even."] pub inclusive : bool , }}}
mkitem!{mkenum!{# [doc = " Decoded unsigned value."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum FullDecoded { # [doc = " Not-a-number."] Nan , # [doc = " Infinities, either positive or negative."] Infinite , # [doc = " Zero, either positive or negative."] Zero , # [doc = " Finite numbers with further decoded fields."] Finite (Decoded) , }}}
mkitem!{mktrait!{# [doc = " A floating point type which can be `decode`d."] pub trait DecodableFloat : RawFloat + Copy { # [doc = " The minimum positive normalized value."] fn min_pos_norm_value () -> Self ; }}}
mkitem!{mkimpl!{# [cfg (target_has_reliable_f16)] impl DecodableFloat for f16 { fn min_pos_norm_value () -> Self { f16 :: MIN_POSITIVE } }}}
mkitem!{mkimpl!{impl DecodableFloat for f32 { fn min_pos_norm_value () -> Self { f32 :: MIN_POSITIVE } }}}
mkitem!{mkimpl!{impl DecodableFloat for f64 { fn min_pos_norm_value () -> Self { f64 :: MIN_POSITIVE } }}}

macro_rules! decode_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode in module {}", module_path!());
    };
}

mkfn!{
    decode_introspect!();
    # [doc = " Returns a sign (true when negative) and `FullDecoded` value"] # [doc = " from given floating point number."] pub fn decode < T : DecodableFloat > (v : T) -> (bool , FullDecoded) { let (mant , exp , sign) = v . integer_decode () ; let even = (mant & 1) == 0 ; let decoded = match v . classify () { FpCategory :: Nan => FullDecoded :: Nan , FpCategory :: Infinite => FullDecoded :: Infinite , FpCategory :: Zero => FullDecoded :: Zero , FpCategory :: Subnormal => { FullDecoded :: Finite (Decoded { mant , minus : 1 , plus : 1 , exp , inclusive : even }) } FpCategory :: Normal => { let minnorm = < T as DecodableFloat > :: min_pos_norm_value () . integer_decode () ; if mant == minnorm . 0 { FullDecoded :: Finite (Decoded { mant : mant << 2 , minus : 1 , plus : 2 , exp : exp - 2 , inclusive : even , }) } else { FullDecoded :: Finite (Decoded { mant : mant << 1 , minus : 1 , plus : 1 , exp : exp - 1 , inclusive : even , }) } } } ; (sign < 0 , decoded) }
}