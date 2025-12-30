// Generated macro for ConstScalar (enum)
macro_rules! DepcrateConstScalar {
() => {
// Module: crate
// Provides: {"ConstScalar"}
// Dependencies: {}
# [doc = " A concrete constant value"] # [derive (Debug , Clone , PartialEq , Eq)] pub enum ConstScalar { Bytes (Box < [u8] > , MemoryMap) , UnevaluatedConst (GeneralConstId , Substitution) , # [doc = " Case of an unknown value that rustc might know but we don't"] Unknown , }
};
}
