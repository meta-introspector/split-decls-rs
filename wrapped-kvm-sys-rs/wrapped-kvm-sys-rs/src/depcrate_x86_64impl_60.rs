// Generated macro for impl_60 (impl)
macro_rules! Depcrate_x86_64impl_60 {
() => {
// Module: crate::x86_64
// Provides: {"impl_60"}
// Dependencies: {}
impl CpuidHandle { pub fn new (nent : u32) -> CpuidHandle { unsafe { let sz = mem :: size_of :: < Cpuid2 > () + nent as usize * mem :: size_of :: < CpuidEntry2 > () ; let ptr = calloc (1 , sz) as * mut Cpuid2 ; assert ! (! ptr . is_null ()) ; (* ptr) . nent = nent ; CpuidHandle { cpuid : ptr } } } }
};
}
