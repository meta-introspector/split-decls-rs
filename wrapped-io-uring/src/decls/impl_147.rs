macro_rules! deps {
    () => {
        Entry128!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl Debug for Entry128 { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Entry128") . field ("op_code" , & self . 0 . 0 . opcode) . field ("flags" , & self . 0 . 0 . flags) . field ("user_data" , & self . 0 . 0 . user_data) . finish () } }
    };
}

impl_147!()