macro_rules! deps {
    () => {
        RepeatN!();
        RepeatNProducer!();
    };
}

macro_rules! impl_804 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for RepeatN < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut dbg = f . debug_struct ("RepeatN") ; if let RepeatNProducer :: Repeats (element , count) = & self . inner { dbg . field ("count" , & count . get ()) . field ("element" , element) . finish () } else { dbg . field ("count" , & 0usize) . finish_non_exhaustive () } } }
    };
}

impl_804!()