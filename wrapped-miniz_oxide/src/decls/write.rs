macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! write {
    () => {
        deps!();
        fn write (src : & [u8] , dst : & mut [u8] , dst_pos : & mut usize) -> Result < () > { match dst . get_mut (* dst_pos .. * dst_pos + src . len ()) { Some (s) => s . copy_from_slice (src) , None => return Err (Error { }) , } * dst_pos += src . len () ; Ok (()) }
    };
}

write!()