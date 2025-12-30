// Generated macro for UsedBy (enum)
macro_rules! Depcrate_attrs_data_structuresUsedBy {
() => {
// Module: crate::attrs::data_structures
// Provides: {"UsedBy"}
// Dependencies: {}
# [doc = " There are three valid forms of the attribute:"] # [doc = " `#[used]`, which is semantically equivalent to `#[used(linker)]` except that the latter is currently unstable."] # [doc = " `#[used(compiler)]`"] # [doc = " `#[used(linker)]`"] # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum UsedBy { Compiler , Linker , }
};
}
