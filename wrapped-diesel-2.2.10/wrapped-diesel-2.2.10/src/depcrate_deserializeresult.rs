// Generated macro for Result (type)
macro_rules! Depcrate_deserializeResult {
() => {
// Module: crate::deserialize
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A specialized result type representing the result of deserializing"] # [doc = " a value from the database."] pub type Result < T > = result :: Result < T , Box < dyn Error + Send + Sync > > ;
};
}
