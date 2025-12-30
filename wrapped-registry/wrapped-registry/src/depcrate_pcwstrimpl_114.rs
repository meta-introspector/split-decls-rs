// Generated macro for impl_114 (impl)
macro_rules! Depcrate_pcwstrimpl_114 {
() => {
// Module: crate::pcwstr
// Provides: {"impl_114"}
// Dependencies: {}
impl OwnedPcwstr { pub fn as_ptr (& self) -> * const u16 { debug_assert ! (self . 0 . last () == Some (& 0) , "`OwnedPcwstr` isn't null-terminated") ; self . 0 . as_ptr () } pub fn as_bytes (& self) -> & [u8] { unsafe { core :: slice :: from_raw_parts (self . as_ptr () as * const _ , self . 0 . len () * 2) } } pub fn as_raw (& self) -> PCWSTR { PCWSTR (self . as_ptr ()) } }
};
}
