// Generated macro for impl_1208 (impl)
macro_rules! Depcrate_test_runner_result_cacheimpl_1208 {
() => {
// Module: crate::test_runner::result_cache
// Provides: {"impl_1208"}
// Dependencies: {}
# [cfg (feature = "std")] impl ResultCache for BasicResultCache { fn key (& self , val : & ResultCacheKey) -> u64 { use std :: collections :: hash_map :: DefaultHasher ; use std :: hash :: Hasher ; use std :: io :: { self , Write } ; struct HashWriter (DefaultHasher) ; impl io :: Write for HashWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } } let mut hash = HashWriter (DefaultHasher :: default ()) ; write ! (hash , "{:?}" , val) . expect ("Debug format returned Err") ; hash . 0 . finish () } fn put (& mut self , key : u64 , result : & TestCaseResult) { self . entries . insert (key , result . clone ()) ; } fn get (& self , key : u64) -> Option < & TestCaseResult > { self . entries . get (& key) } }
};
}
