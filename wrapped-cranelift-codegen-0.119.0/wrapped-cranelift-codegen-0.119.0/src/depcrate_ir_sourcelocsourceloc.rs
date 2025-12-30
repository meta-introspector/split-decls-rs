// Generated macro for SourceLoc (struct)
macro_rules! Depcrate_ir_sourcelocSourceLoc {
() => {
// Module: crate::ir::sourceloc
// Provides: {"SourceLoc"}
// Dependencies: {}
# [doc = " A source location."] # [doc = ""] # [doc = " This is an opaque 32-bit number attached to each Cranelift IR instruction. Cranelift does not"] # [doc = " interpret source locations in any way, they are simply preserved from the input to the output."] # [doc = ""] # [doc = " The default source location uses the all-ones bit pattern `!0`. It is used for instructions"] # [doc = " that can't be given a real source location."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct SourceLoc (u32) ;
};
}
