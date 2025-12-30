// Generated macro for impl_278 (impl)
macro_rules! Depcrateimpl_278 {
() => {
// Module: crate
// Provides: {"impl_278"}
// Dependencies: {}
impl core :: fmt :: Display for GetDisjointMutError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let msg = match self { GetDisjointMutError :: IndexOutOfBounds => "an index is out of bounds" , GetDisjointMutError :: OverlappingIndices => "there were overlapping indices" , } ; core :: fmt :: Display :: fmt (msg , f) } }
};
}
