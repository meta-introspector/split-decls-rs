// Generated macro for worker_thread_logic (function)
macro_rules! Depcrate_enc_lzma2_writer_mtworker_thread_logic {
() => {
// Module: crate::enc::lzma2_writer_mt
// Provides: {"worker_thread_logic"}
// Dependencies: {}
# [doc = " The logic for a single worker thread."] fn worker_thread_logic (worker_handle : WorkerHandle < (u64 , WorkUnit) > , result_tx : SyncSender < (u64 , Vec < u8 >) > , shutdown_flag : Arc < AtomicBool > , error_store : Arc < Mutex < Option < io :: Error > > > , active_workers : Arc < AtomicU32 > ,) { while ! shutdown_flag . load (Ordering :: Acquire) { let (index , work_unit) = match worker_handle . steal () { Some (work) => { active_workers . fetch_add (1 , Ordering :: Release) ; work } None => { break ; } } ; let mut compressed_buffer = Vec :: new () ; let mut writer = Lzma2Writer :: new (& mut compressed_buffer , work_unit . options) ; let result = match writer . write_all (& work_unit . data) { Ok (_) => match writer . flush () { Ok (_) => compressed_buffer , Err (error) => { active_workers . fetch_sub (1 , Ordering :: Release) ; set_error (error , & error_store , & shutdown_flag) ; return ; } } , Err (error) => { active_workers . fetch_sub (1 , Ordering :: Release) ; set_error (error , & error_store , & shutdown_flag) ; return ; } } ; if result_tx . send ((index , result)) . is_err () { active_workers . fetch_sub (1 , Ordering :: Release) ; return ; } active_workers . fetch_sub (1 , Ordering :: Release) ; } }
};
}
