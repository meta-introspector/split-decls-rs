// Generated macro for impl_2652 (impl)
macro_rules! Depcrate_isa_pulley_sharedimpl_2652 {
() => {
// Module: crate::isa::pulley_shared
// Provides: {"impl_2652"}
// Dependencies: {}
impl < P > core :: fmt :: Debug for PulleyBackend < P > where P : PulleyTargetKind , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let PulleyBackend { pulley_target : _ , triple , flags : _ , isa_flags : _ , } = self ; f . debug_struct ("PulleyBackend") . field ("triple" , triple) . finish_non_exhaustive () } }
};
}
