macro_rules! deps {
    () => {
        Bucket!();
    };
}

macro_rules! unlock_bucket_pair {
    () => {
        deps!();
        # [doc = " Unlock a pair of buckets"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Both buckets must be locked"] # [inline] unsafe fn unlock_bucket_pair (bucket1 : & Bucket , bucket2 : & Bucket) { bucket1 . mutex . unlock () ; if ! ptr :: eq (bucket1 , bucket2) { bucket2 . mutex . unlock () ; } }
    };
}

unlock_bucket_pair!();