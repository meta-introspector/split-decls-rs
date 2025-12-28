macro_rules! check_blob {
    () => {
        fn check_blob < F > (db : impl Exists , oid : & ObjectId , mut missing_cb : F) where F : FnMut (& ObjectId , Kind) , { if ! db . exists (oid) { missing_cb (oid , Kind :: Blob) ; } }
    };
}

check_blob!();