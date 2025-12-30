// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
# [cfg (feature = "bincode")] impl < T : ? Sized + serde :: Serialize > Encode for T { fn encode < W : Write > (& self , writer : W) -> Result < () > { bincode :: serialize_into :: < W , T > (writer , self) } }
};
}
