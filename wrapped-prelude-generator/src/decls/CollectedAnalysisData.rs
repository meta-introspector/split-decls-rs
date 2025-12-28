macro_rules! deps {
    () => {
        ExpressionInfo!();
        ImplLatticeInfo!();
        EnumLatticeInfo!();
        StructLatticeInfo!();
    };
}

macro_rules! CollectedAnalysisData {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize , Clone , Default)] pub struct CollectedAnalysisData { pub expressions : HashMap < String , ExpressionInfo > , pub struct_lattices : HashMap < String , StructLatticeInfo > , pub enum_lattices : HashMap < String , EnumLatticeInfo > , pub impl_lattices : HashMap < String , ImplLatticeInfo > , }
    };
}

CollectedAnalysisData!()