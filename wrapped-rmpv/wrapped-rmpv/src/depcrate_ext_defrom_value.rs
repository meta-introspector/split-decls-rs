// Generated macro for from_value (function)
macro_rules! Depcrate_ext_defrom_value {
() => {
// Module: crate::ext::de
// Provides: {"from_value"}
// Dependencies: {}
# [inline] pub fn from_value < T > (val : Value) -> Result < T , Error > where T : for < 'de > Deserialize < 'de > { deserialize_from (val) }
};
}
