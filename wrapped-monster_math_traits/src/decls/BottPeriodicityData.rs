macro_rules! deps {
    () => {
        BottPeriodicityTrait!();
    };
}

macro_rules! BottPeriodicityData {
    () => {
        deps!();
        # [doc = " Struct to hold data for BottPeriodicityTrait implementations."] # [derive (Debug , Clone , PartialEq , Serialize , Deserialize , Default)] pub struct BottPeriodicityData { pub period : String , pub phi_signature : u64 , pub monster_element : u64 , }
    };
}

BottPeriodicityData!()