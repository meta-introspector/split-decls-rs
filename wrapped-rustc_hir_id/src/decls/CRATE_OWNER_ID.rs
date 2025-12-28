macro_rules! deps {
    () => {
        OwnerId!();
    };
}

macro_rules! CRATE_OWNER_ID {
    () => {
        deps!();
        pub const CRATE_OWNER_ID : OwnerId = OwnerId { def_id : CRATE_DEF_ID } ;
    };
}

CRATE_OWNER_ID!();