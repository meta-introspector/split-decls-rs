macro_rules! deps {
    () => {
        MonsterConstants!();
    };
}

macro_rules! BottPeriodicityTrait {
    () => {
        deps!();
        # [doc = " Auto-generated trait for BottPeriodicity"] # [doc = " Phi signature: 18774"] pub trait BottPeriodicityTrait { fn get_period (& self) -> & str ; fn set_period (& mut self , value : String) ; fn test_fixed_point_convergence (& self) ; fn test_mathematical_structure_extraction (& self) ; fn phi_signature (& self) -> u64 ; fn monster_element (& self , constants : & dyn MonsterConstants) -> u64 ; }
    };
}

BottPeriodicityTrait!()