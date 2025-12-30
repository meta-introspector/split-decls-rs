// Generated macro for check_blob (function)
macro_rules! Depcratecheck_blob {
() => {
// Module: crate
// Provides: {"check_blob"}
// Dependencies: {}
fn check_blob < F > (db : impl Exists , oid : & ObjectId , mut missing_cb : F) where F : FnMut (& ObjectId , Kind) , { if ! db . exists (oid) { missing_cb (oid , Kind :: Blob) ; } }
};
}
