macro_rules! deps {
    () => {
        WaitFd!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl fmt :: Debug for WaitFd { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("WaitFd") . field ("fd" , & self . inner . fd) . field ("events" , & self . inner . fd) . field ("revents" , & self . inner . fd) . finish () } }
    };
}

impl_150!()