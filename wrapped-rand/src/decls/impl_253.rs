macro_rules! deps {
    () => {
        ThreadRng!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        # [doc = " Debug implementation does not leak internal state"] impl fmt :: Debug for ThreadRng { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { write ! (fmt , "ThreadRng {{ .. }}") } }
    };
}

impl_253!();