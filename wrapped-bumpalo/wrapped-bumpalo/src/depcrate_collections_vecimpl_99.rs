// Generated macro for impl_99 (impl)
macro_rules! Depcrate_collections_vecimpl_99 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'bump , T , I > Index < I > for Vec < 'bump , T > where I : :: core :: slice :: SliceIndex < [T] > , { type Output = I :: Output ; # [inline] fn index (& self , index : I) -> & Self :: Output { Index :: index (& * * self , index) } }
};
}
