macro_rules! deps {
    () => {
        SlimMaskBuilder!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl Debug for SlimMaskBuilder { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let (mut parts_lo , mut parts_hi) = (vec ! [] , vec ! []) ; for i in 0 .. 32 { parts_lo . push (format ! ("{:02}: {:08b}" , i , self . lo [i])) ; parts_hi . push (format ! ("{:02}: {:08b}" , i , self . hi [i])) ; } f . debug_struct ("SlimMaskBuilder") . field ("lo" , & parts_lo) . field ("hi" , & parts_hi) . finish () } }
    };
}

impl_171!();