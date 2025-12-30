// Generated macro for impl_20 (impl)
macro_rules! Depcrate_appimpl_20 {
() => {
// Module: crate::app
// Provides: {"impl_20"}
// Dependencies: {}
impl < T > StatefulList < T > { pub fn with_items (items : Vec < T >) -> Self { Self { state : ListState :: default () , items , } } pub fn next (& mut self) { let i = match self . state . selected () { Some (i) => { if i >= self . items . len () - 1 { 0 } else { i + 1 } } None => 0 , } ; self . state . select (Some (i)) ; } pub fn previous (& mut self) { let i = match self . state . selected () { Some (i) => { if i == 0 { self . items . len () - 1 } else { i - 1 } } None => 0 , } ; self . state . select (Some (i)) ; } }
};
}
