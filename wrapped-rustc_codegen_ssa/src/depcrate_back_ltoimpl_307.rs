// Generated macro for impl_307 (impl)
macro_rules! Depcrate_back_ltoimpl_307 {
() => {
// Module: crate::back::lto
// Provides: {"impl_307"}
// Dependencies: {}
impl < M : ModuleBufferMethods > SerializedModule < M > { pub fn data (& self) -> & [u8] { match * self { SerializedModule :: Local (ref m) => m . data () , SerializedModule :: FromRlib (ref m) => m , SerializedModule :: FromUncompressedFile (ref m) => m , } } }
};
}
