// Generated macro for CRATE_HIR_ID (const)
macro_rules! DepcrateCRATE_HIR_ID {
() => {
// Module: crate
// Provides: {"CRATE_HIR_ID"}
// Dependencies: {}
# [doc = " The `HirId` corresponding to `CRATE_NODE_ID` and `CRATE_DEF_ID`."] pub const CRATE_HIR_ID : HirId = HirId { owner : OwnerId { def_id : CRATE_DEF_ID } , local_id : ItemLocalId :: ZERO } ;
};
}
