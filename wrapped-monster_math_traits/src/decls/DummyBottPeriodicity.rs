macro_rules! deps {
    () => {
        BottPeriodicityData!();
        BottPeriodicityTrait!();
    };
}

macro_rules! DummyBottPeriodicity {
    () => {
        deps!();
        # [doc = " A dummy implementation of `BottPeriodicityTrait` for testing."] # [derive (Debug , Default , Clone , PartialEq , Serialize , Deserialize)] pub struct DummyBottPeriodicity { data : BottPeriodicityData , }
    };
}

DummyBottPeriodicity!();