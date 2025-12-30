// Generated macro for impl_58 (impl)
macro_rules! Depcrate_serimpl_58 {
() => {
// Module: crate::ser
// Provides: {"impl_58"}
// Dependencies: {}
impl Serializer { fn call_to_string < T > (& mut self , v : & T) -> Result < () > where T : ToString , { self . output += & v . to_string () ; Ok (()) } }
};
}
