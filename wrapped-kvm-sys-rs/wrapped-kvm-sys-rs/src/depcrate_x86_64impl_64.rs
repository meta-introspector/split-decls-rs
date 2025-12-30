// Generated macro for impl_64 (impl)
macro_rules! Depcrate_x86_64impl_64 {
() => {
// Module: crate::x86_64
// Provides: {"impl_64"}
// Dependencies: {}
impl Cpuid2 { pub fn entries (& self) -> & [CpuidEntry2] { unsafe { let begin : * const Cpuid2 = self ; let first_ent = (begin as * const u8) . offset (mem :: size_of :: < Cpuid2 > () as isize) ; slice :: from_raw_parts (first_ent as * const _ , self . nent as usize) } } pub fn entries_mut (& mut self) -> & mut [CpuidEntry2] { unsafe { let begin : * mut Cpuid2 = self ; let first_ent = (begin as * mut u8) . offset (mem :: size_of :: < Cpuid2 > () as isize) ; slice :: from_raw_parts_mut (first_ent as * mut _ , self . nent as usize) } } }
};
}
