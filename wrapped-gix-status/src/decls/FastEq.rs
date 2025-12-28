macro_rules! FastEq {
    () => {
        # [doc = " Compares to blobs by comparing their size and oid, and only looks at the file if"] # [doc = " the size matches, therefore it's very fast."] # [derive (Clone)] pub struct FastEq ;
    };
}

FastEq!();