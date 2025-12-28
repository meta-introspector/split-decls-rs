macro_rules! deps {
    () => {
        RemoteProgress!();
        ProgressId!();
    };
}

macro_rules! setup_remote_progress {
    () => {
        deps!();
        fn setup_remote_progress < 'a > (progress : & mut dyn gix_features :: progress :: DynNestedProgress , reader : & mut Box < dyn ExtendedBufRead < 'a > + Unpin + 'a > , should_interrupt : & 'a AtomicBool ,) { reader . set_progress_handler (Some (Box :: new ({ let mut remote_progress = progress . add_child_with_id ("remote" . to_string () , ProgressId :: RemoteProgress . into ()) ; move | is_err : bool , data : & [u8] | { crate :: RemoteProgress :: translate_to_progress (is_err , data , & mut remote_progress) ; if should_interrupt . load (Ordering :: Relaxed) { ProgressAction :: Interrupt } else { ProgressAction :: Continue } } }) as HandleProgress < 'a >)) ; }
    };
}

setup_remote_progress!();