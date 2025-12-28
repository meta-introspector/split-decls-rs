macro_rules! deps {
    () => {
        Iter!();
        Utf8Path!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl fmt :: Debug for Iter < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct DebugHelper < 'a > (& 'a Utf8Path) ; impl fmt :: Debug for DebugHelper < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . 0 . iter ()) . finish () } } f . debug_tuple ("Iter") . field (& DebugHelper (self . as_path ())) . finish () } }
    };
}

impl_30!()