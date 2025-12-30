// Generated macro for BlockCall (struct)
macro_rules! Depcrate_ir_instructionsBlockCall {
() => {
// Module: crate::ir::instructions
// Provides: {"BlockCall"}
// Dependencies: {}
# [doc = " A pair of a Block and its arguments, stored in a single EntityList internally."] # [doc = ""] # [doc = " NOTE: We don't expose either value_to_block or block_to_value outside of this module because"] # [doc = " this operation is not generally safe. However, as the two share the same underlying layout,"] # [doc = " they can be stored in the same value pool."] # [doc = ""] # [doc = " BlockCall makes use of this shared layout by storing all of its contents (a block and its"] # [doc = " argument) in a single EntityList. This is a bit better than introducing a new entity type for"] # [doc = " the pair of a block name and the arguments entity list, as we don't pay any indirection penalty"] # [doc = " to get to the argument values -- they're stored in-line with the block in the same list."] # [doc = ""] # [doc = " The BlockCall::new function guarantees this layout by requiring a block argument that's written"] # [doc = " in as the first element of the EntityList. Any subsequent entries are always assumed to be real"] # [doc = " Values."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct BlockCall { # [doc = " The underlying storage for the BlockCall. The first element of the values EntityList is"] # [doc = " guaranteed to always be a Block encoded as a Value via BlockCall::block_to_value."] # [doc = " Consequently, the values entity list is never empty."] values : entity :: EntityList < Value > , }
};
}
