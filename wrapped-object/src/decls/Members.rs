macro_rules! deps {
    () => {
        AixMemberOffset!();
    };
}

macro_rules! Members {
    () => {
        deps!();
        # [doc = " The list of members in the archive."] # [derive (Debug , Clone , Copy)] enum Members < 'data > { Common { offset : u64 , end_offset : u64 , } , AixBig { index : & 'data [archive :: AixMemberOffset] , } , }
    };
}

Members!()