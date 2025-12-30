// Generated macro for impl_124 (impl)
macro_rules! Depcrateimpl_124 {
() => {
// Module: crate
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "extract_if")] impl < T , const N : usize , F > core :: fmt :: Debug for ExtractIf < '_ , T , N , F > where F : FnMut (& mut T) -> bool , T : core :: fmt :: Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("ExtractIf") . field (& self . vec . as_slice ()) . finish () } }
};
}
