// Generated macro for swapi (macro)
macro_rules! Depcrate_x86_64_sse2swapi {
() => {
// Module: crate::x86_64::sse2
// Provides: {"swapi"}
// Dependencies: {}
macro_rules ! swapi { ($ x : expr , $ i : expr , $ k : expr) => { unsafe { const K : u8 = $ k ; let k = _mm_set1_epi8 (K as i8) ; u128x1_sse2 :: new (_mm_or_si128 (_mm_srli_epi16 (_mm_and_si128 ($ x . x , k) , $ i) , _mm_and_si128 (_mm_slli_epi16 ($ x . x , $ i) , k) ,)) } } ; }
};
}
