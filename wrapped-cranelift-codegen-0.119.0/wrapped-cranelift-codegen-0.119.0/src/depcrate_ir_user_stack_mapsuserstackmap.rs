// Generated macro for UserStackMap (struct)
macro_rules! Depcrate_ir_user_stack_mapsUserStackMap {
() => {
// Module: crate::ir::user_stack_maps
// Provides: {"UserStackMap"}
// Dependencies: {}
# [doc = " A compiled stack map, describing the location of many GC-managed values."] # [doc = ""] # [doc = " A stack map is associated with a particular instruction, and that"] # [doc = " instruction is a safepoint."] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "enable-serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] pub struct UserStackMap { by_type : SmallVec < [(ir :: Type , CompoundBitSet) ; 1] > , sp_to_sized_stack_slots : Option < u32 > , }
};
}
