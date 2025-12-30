// Generated macro for impl_64 (impl)
macro_rules! Depcrate_compileimpl_64 {
() => {
// Module: crate::compile
// Provides: {"impl_64"}
// Dependencies: {}
impl SuffixCache { fn new (size : usize) -> Self { SuffixCache { table : vec ! [SuffixCacheEntry :: default () ; size] , version : 0 , } } fn get (& mut self , key : SuffixCacheKey , pc : InstPtr) -> Option < InstPtr > { let h = self . hash (& key) ; let e = self . table [h] ; if e . key == key && e . version == self . version { Some (e . pc) } else { self . table [h] = SuffixCacheEntry { key : key , pc : pc , version : self . version , } ; None } } fn clear (& mut self) { self . version += 1 ; } fn hash (& self , suffix : & SuffixCacheKey) -> usize { const FNV_PRIME : u64 = 1099511628211 ; let mut h = 14695981039346656037 ; h = (h ^ (suffix . from_inst as u64)) . wrapping_mul (FNV_PRIME) ; h = (h ^ (suffix . start as u64)) . wrapping_mul (FNV_PRIME) ; h = (h ^ (suffix . end as u64)) . wrapping_mul (FNV_PRIME) ; (h as usize) % self . table . len () } }
};
}
