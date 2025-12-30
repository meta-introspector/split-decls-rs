// Generated macro for from_str (function)
macro_rules! Depcrate_defrom_str {
() => {
// Module: crate::de
// Provides: {"from_str"}
// Dependencies: {}
# [doc = " Deserialize an instance of type `T` from a string of XML text."] pub fn from_str < 'de , T > (s : & 'de str) -> Result < T , DeError > where T : Deserialize < 'de > , { let mut de = Deserializer :: from_str (s) ; T :: deserialize (& mut de) }
};
}
