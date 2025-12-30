// Generated macro for hash (function)
macro_rules! Depcratehash {
() => {
// Module: crate
// Provides: {"hash"}
// Dependencies: {}
# [doc = " `key` is from `phf_generator::HashState`."] # [inline] pub fn hash < T : ? Sized + PhfHash > (x : & T , key : & HashKey) -> Hashes { let mut hasher = SipHasher13 :: new_with_keys (0 , * key) ; x . phf_hash (& mut hasher) ; let Hash128 { h1 : lower , h2 : upper , } = hasher . finish128 () ; Hashes { g : (lower >> 32) as u32 , f1 : lower as u32 , f2 : upper as u32 , } }
};
}
