// Generated macro for impl_133 (impl)
macro_rules! Depcrate_easy_handlerimpl_133 {
() => {
// Module: crate::easy::handler
// Provides: {"impl_133"}
// Dependencies: {}
impl < H : fmt :: Debug > fmt :: Debug for Easy2 < H > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Easy") . field ("handle" , & self . inner . handle) . field ("handler" , & self . inner . handler) . finish () } }
};
}
