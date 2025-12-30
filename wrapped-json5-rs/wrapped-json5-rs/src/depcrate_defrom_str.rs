// Generated macro for from_str (function)
macro_rules! Depcrate_defrom_str {
() => {
// Module: crate::de
// Provides: {"from_str"}
// Dependencies: {}
# [doc = " Deserialize an instance of type `T` from a string of JSON5 text. Can fail if the input is"] # [doc = " invalid JSON5, or doesn&rsquo;t match the structure of the target type."] pub fn from_str < 'a , T > (s : & 'a str) -> Result < T > where T : de :: Deserialize < 'a > , { let mut deserializer = Deserializer :: from_str (s) ? ; T :: deserialize (& mut deserializer) }
};
}
