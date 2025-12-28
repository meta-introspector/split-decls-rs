macro_rules! deps {
    () => {
        ErrorRecord!();
        Error!();
    };
}

macro_rules! handle_error {
    () => {
        deps!();
        fn handle_error < E > (err : E , entry_path : & BStr , files : & AtomicUsize , errors : & mut Vec < checkout :: ErrorRecord > , keep_going : bool ,) -> Result < () , E > where E : std :: error :: Error + Send + Sync + 'static , { if keep_going { errors . push (checkout :: ErrorRecord { path : entry_path . into () , error : Box :: new (err) , }) ; files . fetch_add (1 , Ordering :: Relaxed) ; Ok (()) } else { Err (err) } }
    };
}

handle_error!();