macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! impl_536 {
    () => {
        deps!();
        # [cfg (not (feature = "arbitrary_precision"))] impl PartialEq for N { fn eq (& self , other : & Self) -> bool { match (self , other) { (N :: PosInt (a) , N :: PosInt (b)) => a == b , (N :: NegInt (a) , N :: NegInt (b)) => a == b , (N :: Float (a) , N :: Float (b)) => a == b , _ => false , } } }
    };
}

impl_536!();