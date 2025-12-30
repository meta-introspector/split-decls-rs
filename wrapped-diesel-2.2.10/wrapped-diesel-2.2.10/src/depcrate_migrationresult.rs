// Generated macro for Result (type)
macro_rules! Depcrate_migrationResult {
() => {
// Module: crate::migration
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A specialized result type representing the result of"] # [doc = " a migration operation"] pub type Result < T > = std :: result :: Result < T , Box < dyn Error + Send + Sync > > ;
};
}
