// Generated macro for Writable (struct)
macro_rules! Depcrate_machinst_regWritable {
() => {
// Module: crate::machinst::reg
// Provides: {"Writable"}
// Dependencies: {}
# [doc = " A type wrapper that indicates a register type is writable. The"] # [doc = " underlying register can be extracted, and the type wrapper can be"] # [doc = " built using an arbitrary register. Hence, this type-level wrapper"] # [doc = " is not strictly a guarantee. However, \"casting\" to a writable"] # [doc = " register is an explicit operation for which we can"] # [doc = " audit. Ordinarily, internal APIs in the compiler backend should"] # [doc = " take a `Writable<Reg>` whenever the register is written, and the"] # [doc = " usual, frictionless way to get one of these is to allocate a new"] # [doc = " temporary."] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Writable < T > { reg : T , }
};
}
