macro_rules! deps {
    () => {
        Stack!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl fmt :: Debug for Stack { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let offset = self . get_offset () ; write ! (f , "Stack<{:?}, Offset={}>" , self . buf , unsafe { * offset }) } }
    };
}

impl_69!()