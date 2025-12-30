// Generated macro for impl_92 (impl)
macro_rules! Depcrate_mapimpl_92 {
() => {
// Module: crate::map
// Provides: {"impl_92"}
// Dependencies: {}
impl < T , E > ResultExt < T , E > for Result < T , E > { unsafe fn unsafe_map < U > (self , op : unsafe fn (T) -> U) -> Result < U , E > { match self { Ok (t) => Ok (unsafe { op (t) }) , Err (e) => Err (e) , } } }
};
}
