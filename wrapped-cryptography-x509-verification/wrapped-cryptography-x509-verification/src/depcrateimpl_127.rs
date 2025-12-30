// Generated macro for impl_127 (impl)
macro_rules! Depcrateimpl_127 {
() => {
// Module: crate
// Provides: {"impl_127"}
// Dependencies: {}
impl Budget { const DEFAULT_NAME_CONSTRAINT_CHECK_LIMIT : usize = 1 << 20 ; fn new () -> Budget { Budget { name_constraint_checks : Self :: DEFAULT_NAME_CONSTRAINT_CHECK_LIMIT , } } fn name_constraint_check < 'chain , B : CryptoOps > (& mut self) -> ValidationResult < 'chain , () , B > { self . name_constraint_checks = self . name_constraint_checks . checked_sub (1) . ok_or_else (| | { ValidationError :: new (ValidationErrorKind :: FatalError ("Exceeded maximum name constraint check limit" ,)) }) ? ; Ok (()) } }
};
}
