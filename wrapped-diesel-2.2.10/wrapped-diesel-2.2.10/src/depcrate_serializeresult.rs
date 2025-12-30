// Generated macro for Result (type)
macro_rules! Depcrate_serializeResult {
() => {
// Module: crate::serialize
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A specialized result type representing the result of serializing"] # [doc = " a value for the database."] pub type Result = result :: Result < IsNull , Box < dyn Error + Send + Sync > > ;
};
}
