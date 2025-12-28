macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl Debug for Entry { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Entry") . field ("op_code" , & self . 0 . opcode) . field ("flags" , & self . 0 . flags) . field ("user_data" , & self . 0 . user_data) . finish () } }
    };
}

impl_142!()