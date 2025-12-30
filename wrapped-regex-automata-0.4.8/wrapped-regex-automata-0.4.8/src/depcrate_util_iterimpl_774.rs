// Generated macro for impl_774 (impl)
macro_rules! Depcrate_util_iterimpl_774 {
() => {
// Module: crate::util::iter
// Provides: {"impl_774"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'h , F > core :: fmt :: Debug for TryCapturesIter < 'h , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("TryCapturesIter") . field ("it" , & self . it) . field ("caps" , & self . caps) . field ("finder" , & "<closure>") . finish () } }
};
}
