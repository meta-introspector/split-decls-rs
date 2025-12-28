macro_rules! deps {
    () => {
        ArrayString!();
        CapacityError!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < const CAP : usize > FromStr for ArrayString < CAP > { type Err = CapacityError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: from (s) . map_err (CapacityError :: simplify) } }
    };
}

impl_28!();