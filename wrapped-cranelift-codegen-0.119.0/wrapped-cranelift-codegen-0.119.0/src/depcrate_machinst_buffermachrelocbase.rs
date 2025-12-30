// Generated macro for MachRelocBase (struct)
macro_rules! Depcrate_machinst_bufferMachRelocBase {
() => {
// Module: crate::machinst::buffer
// Provides: {"MachRelocBase"}
// Dependencies: {}
# [doc = " A relocation resulting from a compilation."] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "enable-serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] pub struct MachRelocBase < T > { # [doc = " The offset at which the relocation applies, *relative to the"] # [doc = " containing section*."] pub offset : CodeOffset , # [doc = " The kind of relocation."] pub kind : Reloc , # [doc = " The external symbol / name to which this relocation refers."] pub target : T , # [doc = " The addend to add to the symbol value."] pub addend : i64 , }
};
}
