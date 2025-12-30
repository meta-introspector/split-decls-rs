// Generated macro for parse_finite (function)
macro_rules! Depcrate_math_support_hex_floatparse_finite {
() => {
// Module: crate::math::support::hex_float
// Provides: {"parse_finite"}
// Dependencies: {}
const fn parse_finite (b : & [u8] , bits : u32 , sig_bits : u32 , rounding_mode : Round ,) -> Result < (u128 , Status) , HexFloatParseError > { let exp_bits : u32 = bits - sig_bits - 1 ; let max_msb : i32 = (1 << (exp_bits - 1)) - 1 ; let min_lsb : i32 = 1 - max_msb - sig_bits as i32 ; let (mut sig , mut exp) = match parse_hex (b) { Err (e) => return Err (e) , Ok (Parsed { sig : 0 , .. }) => return Ok ((0 , Status :: OK)) , Ok (Parsed { sig , exp }) => (sig , exp) , } ; let mut round_bits = u128_ilog2 (sig) as i32 - sig_bits as i32 ; if exp < min_lsb - round_bits { round_bits = min_lsb - exp ; } let mut status = Status :: OK ; exp += round_bits ; if round_bits > 0 { if round_bits == 1 { sig <<= 1 ; } else if round_bits > 2 { sig = shr_odd_rounding (sig , (round_bits - 2) as u32) ; } if sig & 0b11 != 0 { status = Status :: INEXACT ; } sig = shr2_round (sig , rounding_mode) ; } else if round_bits < 0 { sig <<= - round_bits ; } let uexp = (exp - min_lsb) as u128 ; let uexp = uexp << sig_bits ; debug_assert ! (sig <= 2 << sig_bits) ; let inf = ((1 << exp_bits) - 1) << sig_bits ; let bits = match sig . checked_add (uexp) { Some (bits) if bits < inf => { if status . inexact () && bits < (1 << sig_bits) { status = status . with (Status :: UNDERFLOW) ; } bits } _ => { status = status . with (Status :: OVERFLOW) . with (Status :: INEXACT) ; match rounding_mode { Round :: Positive | Round :: Nearest => inf , Round :: Negative | Round :: Zero => inf - 1 , } } } ; Ok ((bits , status)) }
};
}
