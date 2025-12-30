// Generated macro for tests (module)
macro_rules! Depcrate_ir_sourceloctests {
() => {
// Module: crate::ir::sourceloc
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: ir :: SourceLoc ; use alloc :: string :: ToString ; # [test] fn display () { assert_eq ! (SourceLoc :: default () . to_string () , "@-") ; assert_eq ! (SourceLoc :: new (0) . to_string () , "@0000") ; assert_eq ! (SourceLoc :: new (16) . to_string () , "@0010") ; assert_eq ! (SourceLoc :: new (0xabcdef) . to_string () , "@abcdef") ; } }
};
}
