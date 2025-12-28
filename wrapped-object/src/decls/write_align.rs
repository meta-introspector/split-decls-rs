macro_rules! deps {
    () => {
        WritableBuffer!();
    };
}

macro_rules! write_align {
    () => {
        deps!();
        pub (crate) fn write_align (buffer : & mut dyn WritableBuffer , size : usize) { let new_len = align (buffer . len () , size) ; buffer . resize (new_len) ; }
    };
}

write_align!()