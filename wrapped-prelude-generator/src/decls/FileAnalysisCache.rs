macro_rules! deps {
    () => {
        EnumLatticeInfo!();
        StructLatticeInfo!();
        ImplLatticeInfo!();
        ExpressionInfo!();
    };
}

macro_rules! FileAnalysisCache {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] struct FileAnalysisCache { expressions : HashMap < String , ExpressionInfo > , struct_lattices : HashMap < String , StructLatticeInfo > , enum_lattices : HashMap < String , EnumLatticeInfo > , impl_lattices : HashMap < String , ImplLatticeInfo > , }
    };
}

FileAnalysisCache!()