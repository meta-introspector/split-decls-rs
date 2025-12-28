macro_rules! DiagArgFromDisplay {
    () => {
        pub struct DiagArgFromDisplay < 'a > (pub & 'a dyn fmt :: Display) ;
    };
}

DiagArgFromDisplay!()