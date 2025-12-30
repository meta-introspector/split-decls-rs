// Generated macro for fuzz (module)
macro_rules! Depcrate_bytes_mutfuzz {
() => {
// Module: crate::bytes_mut
// Provides: {"fuzz"}
// Dependencies: {}
# [cfg (all (test , loom))] mod fuzz { use loom :: sync :: Arc ; use loom :: thread ; use super :: BytesMut ; use crate :: Bytes ; # [test] fn bytes_mut_cloning_frozen () { loom :: model (| | { let a = BytesMut :: from (& b"abcdefgh" [..]) . split () . freeze () ; let addr = a . as_ptr () as usize ; let a1 = Arc :: new (a) ; let a2 = a1 . clone () ; let t1 = thread :: spawn (move | | { let b : Bytes = (* a1) . clone () ; assert_eq ! (b . as_ptr () as usize , addr) ; }) ; let t2 = thread :: spawn (move | | { let b : Bytes = (* a2) . clone () ; assert_eq ! (b . as_ptr () as usize , addr) ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; }) ; } }
};
}
