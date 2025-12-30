// Generated macro for make_fuzz_lengths (function)
macro_rules! Depcratemake_fuzz_lengths {
() => {
// Module: crate
// Provides: {"make_fuzz_lengths"}
// Dependencies: {}
const fn make_fuzz_lengths (bits : u32) -> [u8 ; 20] { let mut v = [0u8 ; 20] ; v [0] = 0 ; v [1] = 1 ; v [2] = 2 ; let mut i = 3 ; let mut l = 8 ; loop { if l >= ((bits / 2) as u8) { break ; } v [i] = l - 1 ; i += 1 ; v [i] = l ; i += 1 ; l *= 2 ; } if bits != 8 { v [i] = ((bits / 2) - 1) as u8 ; i += 1 ; } let mid = i ; let mut j = 1 ; loop { v [i] = (bits as u8) - (v [mid - j]) - 1 ; if j == mid { break ; } i += 1 ; j += 1 ; } v }
};
}
