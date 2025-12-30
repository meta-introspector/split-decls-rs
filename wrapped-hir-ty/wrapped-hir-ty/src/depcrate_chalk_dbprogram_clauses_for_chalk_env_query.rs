// Generated macro for program_clauses_for_chalk_env_query (function)
macro_rules! Depcrate_chalk_dbprogram_clauses_for_chalk_env_query {
() => {
// Module: crate::chalk_db
// Provides: {"program_clauses_for_chalk_env_query"}
// Dependencies: {}
pub (crate) fn program_clauses_for_chalk_env_query (db : & dyn HirDatabase , krate : Crate , block : Option < BlockId > , environment : chalk_ir :: Environment < Interner > ,) -> chalk_ir :: ProgramClauses < Interner > { chalk_solve :: program_clauses_for_env (& ChalkContext { db , krate , block } , & environment) }
};
}
