// Generated macro for get_index (function)
macro_rules! Depcrateget_index {
() => {
// Module: crate
// Provides: {"get_index"}
// Dependencies: {}
# [doc = " Return an index into `phf_generator::HashState::map`."] # [doc = ""] # [doc = " * `hash` is from `hash()` in this crate."] # [doc = " * `disps` is from `phf_generator::HashState::disps`."] # [doc = " * `len` is the length of `phf_generator::HashState::map`."] # [inline] pub fn get_index (hashes : & Hashes , disps : & [(u32 , u32)] , len : usize) -> u32 { let (d1 , d2) = disps [(hashes . g % (disps . len () as u32)) as usize] ; displace (hashes . f1 , hashes . f2 , d1 , d2) % (len as u32) }
};
}
