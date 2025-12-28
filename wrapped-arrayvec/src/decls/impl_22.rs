macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        # [doc = " `Write` appends written data to the end of the string."] impl < const CAP : usize > fmt :: Write for ArrayString < CAP > { fn write_char (& mut self , c : char) -> fmt :: Result { self . try_push (c) . map_err (| _ | fmt :: Error) } fn write_str (& mut self , s : & str) -> fmt :: Result { self . try_push_str (s) . map_err (| _ | fmt :: Error) } }
    };
}

impl_22!()