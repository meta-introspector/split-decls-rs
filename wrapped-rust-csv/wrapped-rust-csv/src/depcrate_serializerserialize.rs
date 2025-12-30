// Generated macro for serialize (function)
macro_rules! Depcrate_serializerserialize {
() => {
// Module: crate::serializer
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serialize the given value to the given writer, and return an error if"] # [doc = " anything went wrong."] pub fn serialize < S : Serialize , W : io :: Write > (wtr : & mut Writer < W > , value : S ,) -> Result < () , Error > { value . serialize (& mut SeRecord { wtr }) }
};
}
