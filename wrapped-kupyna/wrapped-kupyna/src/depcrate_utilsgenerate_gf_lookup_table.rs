// Generated macro for generate_gf_lookup_table (function)
macro_rules! Depcrate_utilsgenerate_gf_lookup_table {
() => {
// Module: crate::utils
// Provides: {"generate_gf_lookup_table"}
// Dependencies: {}
const fn generate_gf_lookup_table () -> [[u8 ; 256] ; 256] { let mut table = [[0u8 ; 256] ; 256] ; let mut x = 0 ; while x < 256 { let mut y = 0 ; while y < 256 { table [x] [y] = gf_multiply (x as u8 , y as u8) ; y += 1 ; } x += 1 ; } table }
};
}
