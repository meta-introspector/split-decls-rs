macro_rules! NO_PARENT_IDS {
    () => {
        # [doc = " An empty array of a type usable with the `gix::easy` API to help declaring no parents should be used"] pub const NO_PARENT_IDS : [gix_hash :: ObjectId ; 0] = [] ;
    };
}

NO_PARENT_IDS!()