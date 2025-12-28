macro_rules! ImageHotPatchHashes {
    () => {
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageHotPatchHashes { pub sha256 : [u8 ; 32] , pub sha1 : [u8 ; 20] , }
    };
}

ImageHotPatchHashes!();