// Generated macro for exp (function)
macro_rules! Depcrateexp {
() => {
// Module: crate
// Provides: {"exp"}
// Dependencies: {}
# [doc = " BigUint modular exponentiation by square-and-multiply."] fn exp (base : BigUint , exp : & BigUint , modulus : & BigUint) -> BigUint { let mut ret = BigUint :: one () ; for i in exp . to_bytes_be () . into_iter () . flat_map (| x | (0 .. 8) . rev () . map (move | i | (x >> i) . is_odd ())) { ret = (& ret * & ret) % modulus ; if i { ret = (ret * & base) % modulus ; } } ret }
};
}
