// Generated macro for impl_473 (impl)
macro_rules! Depcrate_contextimpl_473 {
() => {
// Module: crate::context
// Provides: {"impl_473"}
// Dependencies: {}
impl Data { # [doc = " Insert data."] pub fn insert < D : Any + Send + Sync > (& mut self , data : D) { self . 0 . insert (TypeId :: of :: < D > () , Box :: new (data)) ; } pub (crate) fn merge (& mut self , other : Data) { self . 0 . extend (other . 0) ; } }
};
}
