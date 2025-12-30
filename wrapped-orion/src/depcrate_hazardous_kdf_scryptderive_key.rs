// Generated macro for derive_key (function)
macro_rules! Depcrate_hazardous_kdf_scryptderive_key {
() => {
// Module: crate::hazardous::kdf::scrypt
// Provides: {"derive_key"}
// Dependencies: {}
# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " scrypt key derivation function as specified in [RFC 7914](https://datatracker.ietf.org/doc/html/rfc7914.html)."] pub fn derive_key (password : & [u8] , salt : & [u8] , n : u32 , r : u32 , p : u32 , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { if n <= 1 || n & (n - 1) != 0 || ((r as u64) * (p as u64)) >= RP_MAX || r > RP_BLK_MAX / p || r > R_BLK_MAX || n > N_MAX / r || usize :: BITS < 32 { return Err (UnknownCryptoError) ; } let n : usize = n as usize ; let r : usize = r as usize ; let p : usize = p as usize ; let vlen : usize = 32 * n * r ; let mut x = vec ! [0u32 ; 32 * r] ; let mut y = vec ! [0u32 ; 32 * r] ; let mut v = vec ! [0u32 ; vlen] ; let pass = pbkdf2 :: Password :: from_slice (password) ? ; let blen : usize = p * 128 * r ; let mut b = vec ! [0u8 ; blen] ; pbkdf2 :: derive_key (& pass , salt , 1 , & mut b) . inspect_err (| _ | { b . zeroize () ; }) ? ; for i in 0 .. p { smix (& mut b [i * 128 * r ..] , r , n , & mut v , & mut x , & mut y) ; } pbkdf2 :: derive_key (& pass , & b , 1 , dst_out) . inspect_err (| _ | { b . zeroize () ; }) ? ; b . zeroize () ; Ok (()) }
};
}
