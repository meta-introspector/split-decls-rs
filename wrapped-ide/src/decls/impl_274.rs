macro_rules! deps {
    () => {
        InlayHintLabel!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl fmt :: Debug for InlayHintLabel { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (& self . parts) . finish () } }
    };
}

impl_274!();