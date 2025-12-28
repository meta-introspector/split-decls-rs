macro_rules! deps {
    () => {
        RawEntryMut!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl < K : Debug , V : Debug , S , A : Allocator > Debug for RawEntryMut < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { RawEntryMut :: Vacant (ref v) => f . debug_tuple ("RawEntry") . field (v) . finish () , RawEntryMut :: Occupied (ref o) => f . debug_tuple ("RawEntry") . field (o) . finish () , } } }
    };
}

impl_358!();