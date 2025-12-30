// Generated macro for ResultExt (trait)
macro_rules! Depcrate_mapResultExt {
() => {
// Module: crate::map
// Provides: {"ResultExt"}
// Dependencies: {}
pub (crate) trait ResultExt < T , E > { unsafe fn unsafe_map < U > (self , op : unsafe fn (T) -> U) -> Result < U , E > ; }
};
}
