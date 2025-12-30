// Generated macro for impl_109 (impl)
macro_rules! Depcrate_store_vec_implimpl_109 {
() => {
// Module: crate::store::vec_impl
// Provides: {"impl_109"}
// Dependencies: {}
impl < K , V > StoreIntoIterator < K , V > for Vec < (K , V) > { type KeyValueIntoIter = alloc :: vec :: IntoIter < (K , V) > ; # [inline] fn lm_into_iter (self) -> Self :: KeyValueIntoIter { IntoIterator :: into_iter (self) } # [inline] fn lm_extend_end (& mut self , other : Self) { self . extend (other) } # [inline] fn lm_extend_start (& mut self , other : Self) { self . splice (0 .. 0 , other) ; } }
};
}
