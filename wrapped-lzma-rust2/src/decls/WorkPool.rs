macro_rules! deps {
    () => {
        WorkerFunction!();
        WorkPoolState!();
        WorkStealingQueue!();
        Error!();
    };
}

macro_rules! WorkPool {
    () => {
        deps!();
        # [doc = " A generic work pool for the multi threading reader and writer."] pub (crate) struct WorkPool < W , R > { work_queue : WorkStealingQueue < (u64 , W) > , result_rx : Receiver < (u64 , R) > , result_tx : SyncSender < (u64 , R) > , next_index_to_dispatch : u64 , next_index_to_return : u64 , last_sequence_id : Option < u64 > , out_of_order_results : BTreeMap < u64 , R > , shutdown_flag : Arc < AtomicBool > , error_store : Arc < Mutex < Option < io :: Error > > > , state : WorkPoolState , active_workers : Arc < AtomicU32 > , num_workers : u32 , num_work : u64 , worker_handles : Vec < thread :: JoinHandle < () > > , worker_fn : WorkerFunction < W , R > , }
    };
}

WorkPool!()