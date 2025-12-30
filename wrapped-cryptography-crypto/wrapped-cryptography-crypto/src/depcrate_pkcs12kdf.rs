// Generated macro for kdf (function)
macro_rules! Depcrate_pkcs12kdf {
() => {
// Module: crate::pkcs12
// Provides: {"kdf"}
// Dependencies: {}
pub fn kdf (pass : & str , salt : & [u8] , id : u8 , rounds : u64 , key_len : usize , hash_alg : openssl :: hash :: MessageDigest ,) -> Result < Vec < u8 > , openssl :: error :: ErrorStack > { let pass = pass . encode_utf16 () . chain ([0]) . flat_map (| v | v . to_be_bytes ()) . collect :: < Vec < u8 > > () ; let block_size = hash_alg . block_size () ; let d = vec ! [id ; block_size] ; let s_len = block_size * salt . len () . div_ceil (block_size) ; let p_len = block_size * pass . len () . div_ceil (block_size) ; let mut init_key = vec ! [0 ; s_len + p_len] ; for i in 0 .. s_len { init_key [i] = salt [i % salt . len ()] ; } for i in 0 .. p_len { init_key [i + s_len] = pass [i % pass . len ()] ; } let mut result = vec ! [0 ; key_len] ; let mut pos = 0 ; loop { let mut h = openssl :: hash :: Hasher :: new (hash_alg) ? ; h . update (& d) ? ; h . update (& init_key) ? ; let mut a = h . finish () ? ; for _ in 1 .. rounds { let mut h = openssl :: hash :: Hasher :: new (hash_alg) ? ; h . update (& a) ? ; a = h . finish () ? ; } let to_add = a . len () . min (result . len () - pos) ; result [pos .. pos + to_add] . copy_from_slice (& a [.. to_add]) ; pos += to_add ; if pos == result . len () { break ; } let mut b = vec ! [0 ; block_size] ; for i in 0 .. block_size { b [i] = a [i % a . len ()] ; } assert ! (init_key . len () % block_size == 0) ; let mut j = 0 ; while j < init_key . len () { let mut carry = 1u16 ; let mut k = block_size - 1 ; loop { carry += init_key [k + j] as u16 + b [k] as u16 ; init_key [j + k] = carry as u8 ; carry >>= 8 ; if k == 0 { break ; } k -= 1 ; } j += block_size ; } } Ok (result) }
};
}
