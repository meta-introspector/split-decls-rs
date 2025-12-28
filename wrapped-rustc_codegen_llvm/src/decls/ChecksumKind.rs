macro_rules! ChecksumKind {
    () => {
        # [doc = " LLVMRustChecksumKind"] # [derive (Copy , Clone)] # [repr (C)] pub (crate) enum ChecksumKind { None , MD5 , SHA1 , SHA256 , }
    };
}

ChecksumKind!()