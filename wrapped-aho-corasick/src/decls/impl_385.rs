macro_rules! deps {
    () => {
        RareByteOffsets!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        impl core :: fmt :: Debug for RareByteOffsets { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let mut offsets = vec ! [] ; for off in self . set . iter () { if off . max > 0 { offsets . push (off) ; } } f . debug_struct ("RareByteOffsets") . field ("set" , & offsets) . finish () } }
    };
}

impl_385!()