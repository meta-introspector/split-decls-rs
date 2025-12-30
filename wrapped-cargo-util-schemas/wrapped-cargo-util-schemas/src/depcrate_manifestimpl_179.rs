// Generated macro for impl_179 (impl)
macro_rules! Depcrate_manifestimpl_179 {
() => {
// Module: crate::manifest
// Provides: {"impl_179"}
// Dependencies: {}
impl ser :: Serialize for ProfilePackageSpec { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { self . to_string () . serialize (s) } }
};
}
