// Generated macro for serialize (function)
macro_rules! Depcrate_serdeserialize {
() => {
// Module: crate::serde
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serde: Serialize with 0x-prefix and ignore case"] pub fn serialize < S , T > (data : T , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , T : AsRef < [u8] > , { withpfx_ignorecase :: serialize (data , serializer) }
};
}
