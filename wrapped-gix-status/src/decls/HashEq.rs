macro_rules! deps {
    () => {
        FastEq!();
    };
}

macro_rules! HashEq {
    () => {
        deps!();
        # [doc = " Compares files to blobs by *always* comparing their hashes."] # [doc = ""] # [doc = " Same as [`FastEq`] but does not contain a fast path for files with mismatched files and"] # [doc = " therefore always returns an OID that can be reused later."] # [derive (Clone)] pub struct HashEq ;
    };
}

HashEq!();