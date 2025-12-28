macro_rules! deps {
    () => {
        WorkUnit!();
        ResultUnit!();
        Error!();
        WorkerHandle!();
        Lzma2Reader!();
    };
}

macro_rules! worker_thread_logic {
    () => {
        deps!();
        # [doc = " The logic for a single worker thread."] fn worker_thread_logic (worker_handle : WorkerHandle < WorkUnit > , result_tx : SyncSender < ResultUnit > , dict_size : u32 , preset_dict : Option < Arc < Vec < u8 > > > , shutdown_flag : Arc < AtomicBool > , error_store : Arc < Mutex < Option < io :: Error > > > , active_workers : Arc < AtomicU32 > ,) { while ! shutdown_flag . load (Ordering :: Acquire) { let (seq , work_unit_data) = match worker_handle . steal () { Some (work) => { active_workers . fetch_add (1 , Ordering :: Release) ; work } None => { break ; } } ; let mut reader = Lzma2Reader :: new (work_unit_data . as_slice () , dict_size , preset_dict . as_deref () . map (| v | v . as_slice ()) ,) ; let mut decompressed_data = Vec :: with_capacity (work_unit_data . len ()) ; let result = match reader . read_to_end (& mut decompressed_data) { Ok (_) => decompressed_data , Err (error) => { active_workers . fetch_sub (1 , Ordering :: Release) ; set_error (error , & error_store , & shutdown_flag) ; return ; } } ; if result_tx . send ((seq , result)) . is_err () { active_workers . fetch_sub (1 , Ordering :: Release) ; return ; } active_workers . fetch_sub (1 , Ordering :: Release) ; } }
    };
}

worker_thread_logic!();