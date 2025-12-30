// Generated macro for impl_357 (impl)
macro_rules! Depcrate_mapimpl_357 {
() => {
// Module: crate::map
// Provides: {"impl_357"}
// Dependencies: {}
impl < K , Q , V , S , A > Debug for VacantEntryRef < '_ , '_ , K , Q , V , S , A > where K : Borrow < Q > , Q : Debug + ? Sized , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntryRef") . field (& self . key ()) . finish () } }
};
}
