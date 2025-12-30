// Generated macro for hash_elem_using (function)
macro_rules! Depcrate_header_maphash_elem_using {
() => {
// Module: crate::header::map
// Provides: {"hash_elem_using"}
// Dependencies: {}
fn hash_elem_using < K > (danger : & Danger , k : & K) -> HashValue where K : Hash + ? Sized , { use fnv :: FnvHasher ; const MASK : u64 = (MAX_SIZE as u64) - 1 ; let hash = match * danger { Danger :: Red (ref hasher) => { let mut h = hasher . build_hasher () ; k . hash (& mut h) ; h . finish () } _ => { let mut h = FnvHasher :: default () ; k . hash (& mut h) ; h . finish () } } ; HashValue ((hash & MASK) as u16) }
};
}
