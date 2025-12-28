macro_rules! MH_NLIST_OUTOFSYNC_WITH_DYLDINFO {
    () => {
        # [doc = " The external symbols listed in the nlist symbol table do not include all the symbols listed in the dyld info."] pub const MH_NLIST_OUTOFSYNC_WITH_DYLDINFO : u32 = 0x0400_0000 ;
    };
}

MH_NLIST_OUTOFSYNC_WITH_DYLDINFO!()