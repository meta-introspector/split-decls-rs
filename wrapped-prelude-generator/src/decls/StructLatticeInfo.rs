macro_rules! StructLatticeInfo {
    () => {
        # [derive (Debug , Default , Clone , Serialize , Deserialize)] pub struct StructLatticeInfo { pub struct_name : String , pub field_co_occurrences : HashMap < String , usize > , pub total_expressions_analyzed : usize , }
    };
}

StructLatticeInfo!();