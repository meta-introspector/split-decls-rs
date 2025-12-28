macro_rules! deps {
    () => {
        Probe!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl fmt :: Debug for Probe { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct Op < 'a > (& 'a sys :: io_uring_probe_op) ; impl fmt :: Debug for Op < '_ > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Op") . field ("code" , & self . 0 . op) . finish () } } let probe = & (self . 0) . 0 ; let list = unsafe { probe . ops . as_slice (probe . last_op as usize + 1) } ; let list = list . iter () . filter (| op | op . flags & (sys :: IO_URING_OP_SUPPORTED as u16) != 0) . map (Op) ; f . debug_set () . entries (list) . finish () } }
    };
}

impl_121!();