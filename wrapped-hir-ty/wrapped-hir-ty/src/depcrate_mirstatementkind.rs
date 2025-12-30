// Generated macro for StatementKind (enum)
macro_rules! Depcrate_mirStatementKind {
() => {
// Module: crate::mir
// Provides: {"StatementKind"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Clone)] pub enum StatementKind < 'db > { Assign (Place < 'db > , Rvalue < 'db >) , FakeRead (Place < 'db >) , Deinit (Place < 'db >) , StorageLive (LocalId < 'db >) , StorageDead (LocalId < 'db >) , Nop , }
};
}
