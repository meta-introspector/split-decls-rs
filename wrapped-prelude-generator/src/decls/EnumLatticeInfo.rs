macro_rules! EnumLatticeInfo {
    () => {
        # [derive (Debug , Default , Clone , Serialize , Deserialize)] pub struct EnumLatticeInfo { pub enum_name : String , pub variant_type_co_occurrences : HashMap < String , usize > , pub total_expressions_analyzed : usize , }
    };
}

EnumLatticeInfo!();