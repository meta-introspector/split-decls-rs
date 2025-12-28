macro_rules! deps {
    () => {
        CodegenCx!();
        Visibility!();
    };
}

macro_rules! visibility_di_flags {
    () => {
        deps!();
        # [doc = " Returns the `DIFlags` corresponding to the visibility of the item identified by `did`."] # [doc = ""] # [doc = " `DIFlags::Flag{Public,Protected,Private}` correspond to `DW_AT_accessibility`"] # [doc = " (public/protected/private) aren't exactly right for Rust, but neither is `DW_AT_visibility`"] # [doc = " (local/exported/qualified), and there's no way to set `DW_AT_visibility` in LLVM's API."] fn visibility_di_flags < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , did : DefId , type_did : DefId ,) -> DIFlags { let parent_did = cx . tcx . parent (type_did) ; let visibility = cx . tcx . visibility (did) ; match visibility { Visibility :: Public => DIFlags :: FlagPublic , Visibility :: Restricted (did) if did == parent_did => DIFlags :: FlagPrivate , Visibility :: Restricted (..) => DIFlags :: FlagProtected , } }
    };
}

visibility_di_flags!();