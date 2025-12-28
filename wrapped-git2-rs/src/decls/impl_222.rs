macro_rules! deps {
    () => {
        Error!();
        Binding!();
        Oid!();
        BlobWriter!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < 'repo > BlobWriter < 'repo > { # [doc = " Finalize blob writing stream and write the blob to the object db"] pub fn commit (mut self) -> Result < Oid , Error > { self . need_cleanup = false ; let mut raw = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; unsafe { try_call ! (raw :: git_blob_create_fromstream_commit (& mut raw , self . raw)) ; Ok (Binding :: from_raw (& raw as * const _)) } } }
    };
}

impl_222!();