// Generated macro for FunctionStartsIterator (struct)
macro_rules! Depcrate_read_macho_function_startsFunctionStartsIterator {
() => {
// Module: crate::read::macho::function_starts
// Provides: {"FunctionStartsIterator"}
// Dependencies: {}
# [doc = " Iterator over the function starts in a `LC_FUNCTION_STARTS` load command."] # [derive (Debug , Default , Clone , Copy)] pub struct FunctionStartsIterator < 'data > { data : Bytes < 'data > , addr : u64 , }
};
}
