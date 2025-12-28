macro_rules! set_error {
    () => {
        # [doc = " Helper to set the shared error state and trigger shutdown."] # [cfg (feature = "std")] fn set_error (error : Error , error_store : & std :: sync :: Arc < std :: sync :: Mutex < Option < Error > > > , shutdown_flag : & std :: sync :: Arc < std :: sync :: atomic :: AtomicBool > ,) { let mut guard = error_store . lock () . unwrap () ; if guard . is_none () { * guard = Some (error) ; } shutdown_flag . store (true , std :: sync :: atomic :: Ordering :: Release) ; }
    };
}

set_error!()