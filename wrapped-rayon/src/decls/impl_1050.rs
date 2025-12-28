macro_rules! deps {
    () => {
        Producer!();
        IntoIter!();
        OptionProducer!();
    };
}

macro_rules! impl_1050 {
    () => {
        deps!();
        impl < T : Send > Producer for OptionProducer < T > { type Item = T ; type IntoIter = std :: option :: IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { self . opt . into_iter () } fn split_at (self , index : usize) -> (Self , Self) { debug_assert ! (index <= 1) ; let none = OptionProducer { opt : None } ; if index == 0 { (none , self) } else { (self , none) } } }
    };
}

impl_1050!();