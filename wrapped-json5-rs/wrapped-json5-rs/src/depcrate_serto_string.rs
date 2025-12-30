// Generated macro for to_string (function)
macro_rules! Depcrate_serto_string {
() => {
// Module: crate::ser
// Provides: {"to_string"}
// Dependencies: {}
# [doc = " Attempts to serialize the input as a JSON5 string (actually a JSON string)."] pub fn to_string < T > (value : & T) -> Result < String > where T : Serialize , { let mut serializer = Serializer { output : String :: new () , } ; value . serialize (& mut serializer) ? ; Ok (serializer . output) }
};
}
