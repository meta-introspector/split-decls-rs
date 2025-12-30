// Generated macro for unlock_bucket_pair (function)
macro_rules! Depcrate_parking_lotunlock_bucket_pair {
() => {
// Module: crate::parking_lot
// Provides: {"unlock_bucket_pair"}
// Dependencies: {}
# [doc = " Unlock a pair of buckets"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Both buckets must be locked"] # [inline] unsafe fn unlock_bucket_pair (bucket1 : & Bucket , bucket2 : & Bucket) { bucket1 . mutex . unlock () ; if ! ptr :: eq (bucket1 , bucket2) { bucket2 . mutex . unlock () ; } }
};
}
