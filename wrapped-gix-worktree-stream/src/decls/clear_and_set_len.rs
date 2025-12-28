macro_rules! clear_and_set_len {
    () => {
        fn clear_and_set_len (buf : & mut Vec < u8 > , len : usize) -> io :: Result < () > { buf . clear () ; buf . try_reserve (len) . map_err (| _ | ErrorKind :: OutOfMemory) ? ; buf . resize (len , 0) ; Ok (()) }
    };
}

clear_and_set_len!();