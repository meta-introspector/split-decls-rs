macro_rules! deps {
    () => {
        CrateGraphBuilder!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl fmt :: Debug for CrateGraphBuilder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . arena . iter () . map (| (id , data) | (u32 :: from (id . into_raw ()) , data))) . finish () } }
    };
}

impl_18!()