macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! reserve_double_buffer_size {
    () => {
        deps!();
        # [cfg (any (feature = "fs" , feature = "user"))] fn reserve_double_buffer_size < T > (buf : & mut Vec < T > , limit : usize) -> Result < () > { use std :: cmp :: min ; if buf . capacity () >= limit { return Err (Errno :: ERANGE) ; } let capacity = min (buf . capacity () * 2 , limit) ; buf . reserve (capacity) ; Ok (()) }
    };
}

reserve_double_buffer_size!()