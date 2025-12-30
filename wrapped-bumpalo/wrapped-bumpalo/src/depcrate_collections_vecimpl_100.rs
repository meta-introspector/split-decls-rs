// Generated macro for impl_100 (impl)
macro_rules! Depcrate_collections_vecimpl_100 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'bump , T , I > IndexMut < I > for Vec < 'bump , T > where I : :: core :: slice :: SliceIndex < [T] > , { # [inline] fn index_mut (& mut self , index : I) -> & mut Self :: Output { IndexMut :: index_mut (& mut * * self , index) } }
};
}
