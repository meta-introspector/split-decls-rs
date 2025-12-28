macro_rules! deps {
    () => {
        HirId!();
        OwnerId!();
    };
}

macro_rules! CRATE_HIR_ID {
    () => {
        deps!();
        # [doc = " The `HirId` corresponding to `CRATE_NODE_ID` and `CRATE_DEF_ID`."] pub const CRATE_HIR_ID : HirId = HirId { owner : OwnerId { def_id : CRATE_DEF_ID } , local_id : ItemLocalId :: ZERO } ;
    };
}

CRATE_HIR_ID!()