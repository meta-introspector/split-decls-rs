macro_rules! deps {
    () => {
        Ordering!();
        Mode!();
        Store!();
        Handle!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        # [doc = " Handle registration"] impl super :: Store { pub (crate) fn register_handle (& self) -> Mode { self . num_handles_unstable . fetch_add (1 , Ordering :: Relaxed) ; Mode :: DeletedPacksAreInaccessible } pub (crate) fn remove_handle (& self , mode : Mode) { match mode { Mode :: KeepDeletedPacksAvailable => { let _lock = self . write . lock () ; self . num_handles_stable . fetch_sub (1 , Ordering :: SeqCst) } Mode :: DeletedPacksAreInaccessible => self . num_handles_unstable . fetch_sub (1 , Ordering :: Relaxed) , } ; } pub (crate) fn upgrade_handle (& self , mode : Mode) -> Mode { if let Mode :: DeletedPacksAreInaccessible = mode { let _lock = self . write . lock () ; self . num_handles_stable . fetch_add (1 , Ordering :: SeqCst) ; self . num_handles_unstable . fetch_sub (1 , Ordering :: SeqCst) ; } Mode :: KeepDeletedPacksAvailable } }
    };
}

impl_64!();