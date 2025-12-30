// Generated macro for Token (enum)
macro_rules! Depcrate_globToken {
() => {
// Module: crate::glob
// Provides: {"Token"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] enum Token { Literal (char) , Any , ZeroOrMore , RecursivePrefix , RecursiveSuffix , RecursiveZeroOrMore , Class { negated : bool , ranges : Vec < (char , char) > } , Alternates (Vec < Tokens >) , }
};
}
