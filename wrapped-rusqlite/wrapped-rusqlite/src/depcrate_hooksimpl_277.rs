// Generated macro for impl_277 (impl)
macro_rules! Depcrate_hooksimpl_277 {
() => {
// Module: crate::hooks
// Provides: {"impl_277"}
// Dependencies: {}
impl Wal { # [doc = " Checkpoint a database"] pub fn checkpoint (& self) -> Result < () > { unsafe { decode_result_raw (self . db , ffi :: sqlite3_wal_checkpoint (self . db , self . db_name)) } } # [doc = " Checkpoint a database"] pub fn checkpoint_v2 (& self , mode : CheckpointMode) -> Result < (c_int , c_int) > { let mut n_log = 0 ; let mut n_ckpt = 0 ; unsafe { decode_result_raw (self . db , ffi :: sqlite3_wal_checkpoint_v2 (self . db , self . db_name , mode as c_int , & mut n_log , & mut n_ckpt ,) ,) ? } ; Ok ((n_log , n_ckpt)) } # [doc = " Name of the database that was written to"] pub fn name (& self) -> & CStr { unsafe { CStr :: from_ptr (self . db_name) } } }
};
}
