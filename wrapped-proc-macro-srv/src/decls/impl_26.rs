macro_rules! deps {
    () => {
        TopSubtree!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < S : Copy + fmt :: Debug > fmt :: Debug for TopSubtree < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& tt :: TokenTreesView :: new (& self . 0) , f) } }
    };
}

impl_26!();