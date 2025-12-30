// Generated macro for impl_1041 (impl)
macro_rules! Depcrate_util_searchimpl_1041 {
() => {
// Module: crate::util::search
// Provides: {"impl_1041"}
// Dependencies: {}
impl < 'h > core :: fmt :: Debug for Input < 'h > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: util :: escape :: DebugHaystack ; f . debug_struct ("Input") . field ("haystack" , & DebugHaystack (self . haystack ())) . field ("span" , & self . span) . field ("anchored" , & self . anchored) . field ("earliest" , & self . earliest) . finish () } }
};
}
