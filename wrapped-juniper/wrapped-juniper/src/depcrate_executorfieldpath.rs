// Generated macro for FieldPath (enum)
macro_rules! Depcrate_executorFieldPath {
() => {
// Module: crate::executor
// Provides: {"FieldPath"}
// Dependencies: {}
# [expect (missing_docs , reason = "self-explanatory")] # [derive (Clone)] pub enum FieldPath < 'a > { Root (SourcePosition) , Field (& 'a str , SourcePosition , Arc < FieldPath < 'a > >) , }
};
}
