// Generated macro for impl_146 (impl)
macro_rules! Depcrate_definitionsimpl_146 {
() => {
// Module: crate::definitions
// Provides: {"impl_146"}
// Dependencies: {}
impl DisambiguatorState { pub fn new () -> Self { Self { next : Default :: default () } } # [doc = " Creates a `DisambiguatorState` where the next allocated `(LocalDefId, DefPathData)` pair"] # [doc = " will have `index` as the disambiguator."] pub fn with (def_id : LocalDefId , data : DefPathData , index : u32) -> Self { let mut this = Self :: new () ; this . next . insert ((def_id , data) , index) ; this } }
};
}
