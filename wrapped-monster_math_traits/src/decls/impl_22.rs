macro_rules! deps {
    () => {
        DummyBottPeriodicity!();
        BottPeriodicityData!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl DummyBottPeriodicity { pub fn new (period : String , phi_signature : u64 , monster_element : u64) -> Self { Self { data : BottPeriodicityData { period , phi_signature , monster_element , } , } } }
    };
}

impl_22!()