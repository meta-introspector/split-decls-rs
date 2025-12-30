// Generated macro for make_utf16_invalid (function)
macro_rules! Depcrate_arbitrary__std_envmake_utf16_invalid {
() => {
// Module: crate::arbitrary::_std::env
// Provides: {"make_utf16_invalid"}
// Dependencies: {}
# [cfg (any (target_os = "windows" , test))] fn make_utf16_invalid (buf : & mut [u16] , p : usize) { assert ! (buf . len () > 0) ; let gen_trail = 0 == p || 0xd800 != (buf [p - 1] & 0xfc00) ; let gen_lead = p == buf . len () - 1 || 0xdc00 != (buf [p + 1] & 0xfc00) ; let (force_bits_mask , force_bits_value) = if gen_trail { if gen_lead { (0xf800 , 0xd800) } else { (0xfc00 , 0xdc00) } } else { (0xfc00 , 0xd800) } ; debug_assert_eq ! (0 , (force_bits_value & ! force_bits_mask)) ; buf [p] = (buf [p] & ! force_bits_mask) | force_bits_value ; }
};
}
