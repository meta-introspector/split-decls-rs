// Generated macro for impl_281 (impl)
macro_rules! Depcrate_errorimpl_281 {
() => {
// Module: crate::error
// Provides: {"impl_281"}
// Dependencies: {}
impl core :: fmt :: Debug for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { if ! f . alternate () { core :: fmt :: Display :: fmt (self , f) } else { let Some (ref inner) = self . inner else { return f . debug_struct ("Error") . field ("kind" , & "None") . finish () ; } ; # [cfg (feature = "alloc")] { f . debug_struct ("Error") . field ("kind" , & inner . kind) . field ("cause" , & inner . cause) . finish () } # [cfg (not (feature = "alloc"))] { f . debug_struct ("Error") . field ("kind" , & inner . kind) . finish () } } } }
};
}
