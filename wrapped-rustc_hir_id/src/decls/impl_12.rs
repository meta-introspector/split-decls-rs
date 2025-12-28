macro_rules! deps {
    () => {
        HirId!();
        OwnerId!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl HirId { # [doc = " Signal local id which should never be used."] pub const INVALID : HirId = HirId { owner : OwnerId { def_id : CRATE_DEF_ID } , local_id : ItemLocalId :: INVALID } ; # [inline] pub fn expect_owner (self) -> OwnerId { assert_eq ! (self . local_id . index () , 0) ; self . owner } # [inline] pub fn as_owner (self) -> Option < OwnerId > { if self . local_id . index () == 0 { Some (self . owner) } else { None } } # [inline] pub fn is_owner (self) -> bool { self . local_id . index () == 0 } # [inline] pub fn make_owner (owner : LocalDefId) -> Self { Self { owner : OwnerId { def_id : owner } , local_id : ItemLocalId :: ZERO } } }
    };
}

impl_12!();