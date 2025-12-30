// Generated macro for impl_493 (impl)
macro_rules! Depcrate_util_searchimpl_493 {
() => {
// Module: crate::util::search
// Provides: {"impl_493"}
// Dependencies: {}
impl < 'h > core :: fmt :: Debug for Input < 'h > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let mut fmter = f . debug_struct ("Input") ; match core :: str :: from_utf8 (self . haystack ()) { Ok (nice) => fmter . field ("haystack" , & nice) , Err (_) => fmter . field ("haystack" , & self . haystack ()) , } . field ("span" , & self . span) . field ("anchored" , & self . anchored) . field ("earliest" , & self . earliest) . finish () } }
};
}
