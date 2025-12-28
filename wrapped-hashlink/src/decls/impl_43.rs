macro_rules! deps {
    () => {
        RawEntryMut!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V : fmt :: Debug , S > fmt :: Debug for RawEntryMut < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { RawEntryMut :: Vacant (ref v) => f . debug_tuple ("RawEntry") . field (v) . finish () , RawEntryMut :: Occupied (ref o) => f . debug_tuple ("RawEntry") . field (o) . finish () , } } }
    };
}

impl_43!()