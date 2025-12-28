macro_rules! IoWriter {
    () => {
        pub (crate) struct IoWriter < 'a , W : std :: io :: Write > { writer : & 'a mut W , bytes_written : usize , }
    };
}

IoWriter!();