macro_rules! ImplLatticeInfo {
    () => {
        # [derive (Debug , Default , Clone , Serialize , Deserialize)] pub struct ImplLatticeInfo { pub impl_for_type : String , pub method_co_occurrences : HashMap < String , usize > , pub total_expressions_analyzed : usize , }
    };
}

ImplLatticeInfo!();