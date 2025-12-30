// Generated macro for impl_52 (impl)
macro_rules! Depcrate_eventsimpl_52 {
() => {
// Module: crate::events
// Provides: {"impl_52"}
// Dependencies: {}
impl EventImportance { # [doc = " Returns true if this importance level is included by `other`."] pub fn is_contained_in (& self , other : & EventImportance) -> bool { match (other , self) { (EventImportance :: Core , EventImportance :: Core) => true , (EventImportance :: Base , EventImportance :: Core) | (EventImportance :: Base , EventImportance :: Base) => true , (EventImportance :: Extra , EventImportance :: Core) | (EventImportance :: Extra , EventImportance :: Base) | (EventImportance :: Extra , EventImportance :: Extra) => true , (..) => false , } } }
};
}
