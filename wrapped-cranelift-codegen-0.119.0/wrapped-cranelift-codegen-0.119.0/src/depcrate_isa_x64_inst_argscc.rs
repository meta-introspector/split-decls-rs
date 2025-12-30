// Generated macro for CC (enum)
macro_rules! Depcrate_isa_x64_inst_argsCC {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"CC"}
// Dependencies: {}
# [doc = " These indicate condition code tests.  Not all are represented since not all are useful in"] # [doc = " compiler-generated code."] # [derive (Copy , Clone , PartialEq , Eq)] # [repr (u8)] pub enum CC { # [doc = "  overflow"] O = 0 , # [doc = " no overflow"] NO = 1 , # [doc = " < unsigned"] B = 2 , # [doc = " >= unsigned"] NB = 3 , # [doc = " zero"] Z = 4 , # [doc = " not-zero"] NZ = 5 , # [doc = " <= unsigned"] BE = 6 , # [doc = " > unsigned"] NBE = 7 , # [doc = " negative"] S = 8 , # [doc = " not-negative"] NS = 9 , # [doc = " < signed"] L = 12 , # [doc = " >= signed"] NL = 13 , # [doc = " <= signed"] LE = 14 , # [doc = " > signed"] NLE = 15 , # [doc = " parity"] P = 10 , # [doc = " not parity"] NP = 11 , }
};
}
