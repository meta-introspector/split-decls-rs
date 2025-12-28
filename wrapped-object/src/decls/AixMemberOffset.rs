macro_rules! AixMemberOffset {
    () => {
        # [doc = " Offset of a member in an AIX big archive."] # [doc = ""] # [doc = " This is used in the member index."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct AixMemberOffset (pub [u8 ; 20]) ;
    };
}

AixMemberOffset!()