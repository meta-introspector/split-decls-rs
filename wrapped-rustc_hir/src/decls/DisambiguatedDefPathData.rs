macro_rules! deps {
    () => {
        DefPathData!();
    };
}

macro_rules! DisambiguatedDefPathData {
    () => {
        deps!();
        # [doc = " A pair of `DefPathData` and an integer disambiguator. The integer is"] # [doc = " normally `0`, but in the event that there are multiple defs with the"] # [doc = " same `parent` and `data`, we use this field to disambiguate"] # [doc = " between them. This introduces some artificial ordering dependency"] # [doc = " but means that if you have, e.g., two impls for the same type in"] # [doc = " the same module, they do get distinct `DefId`s."] # [derive (Copy , Clone , PartialEq , Debug , Encodable , Decodable)] pub struct DisambiguatedDefPathData { pub data : DefPathData , pub disambiguator : u32 , }
    };
}

DisambiguatedDefPathData!();