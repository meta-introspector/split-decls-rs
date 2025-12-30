// Generated macro for impl_52 (impl)
macro_rules! Depcrate_isaac_arrayimpl_52 {
() => {
// Module: crate::isaac_array
// Provides: {"impl_52"}
// Dependencies: {}
impl < T > :: core :: default :: Default for IsaacArray < T > where T : Copy + Default , { fn default () -> IsaacArray < T > { IsaacArray { inner : [T :: default () ; RAND_SIZE] , } } }
};
}
