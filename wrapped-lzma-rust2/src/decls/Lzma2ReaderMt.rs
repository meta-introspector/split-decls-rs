macro_rules! deps {
    () => {
        State!();
        Read!();
        WorkUnit!();
        WorkStealingQueue!();
        Error!();
        ResultUnit!();
    };
}

macro_rules! Lzma2ReaderMt {
    () => {
        deps!();
        # [doc = " A multi-threaded LZMA2 decompressor."] pub struct Lzma2ReaderMt < R : Read > { inner : R , result_rx : Receiver < ResultUnit > , result_tx : SyncSender < ResultUnit > , current_work_unit : Vec < u8 > , next_sequence_to_dispatch : u64 , next_sequence_to_return : u64 , last_sequence_id : Option < u64 > , out_of_order_chunks : BTreeMap < u64 , Vec < u8 > > , current_chunk : Cursor < Vec < u8 > > , shutdown_flag : Arc < AtomicBool > , error_store : Arc < Mutex < Option < io :: Error > > > , state : State , work_queue : WorkStealingQueue < WorkUnit > , active_workers : Arc < AtomicU32 > , max_workers : u32 , dict_size : u32 , preset_dict : Option < Arc < Vec < u8 > > > , worker_handles : Vec < thread :: JoinHandle < () > > , }
    };
}

Lzma2ReaderMt!();