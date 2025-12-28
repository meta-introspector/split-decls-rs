macro_rules! deps {
    () => {
        EntryRef!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < K , Q , V , S , A > Debug for EntryRef < '_ , '_ , K , Q , V , S , A > where K : Debug + Borrow < Q > , Q : Debug + ? Sized , V : Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { EntryRef :: Vacant (ref v) => f . debug_tuple ("EntryRef") . field (v) . finish () , EntryRef :: Occupied (ref o) => f . debug_tuple ("EntryRef") . field (o) . finish () , } } }
    };
}

impl_288!();