macro_rules! deps {
    () => {
        StructLatticeInfo!();
        EnumLatticeInfo!();
        ImplLatticeInfo!();
        ExpressionInfo!();
    };
}

macro_rules! TypeUsageVisitor {
    () => {
        deps!();
        pub struct TypeUsageVisitor { pub max_depth : usize , pub current_depth : usize , pub expressions : HashMap < String , ExpressionInfo > , pub struct_lattices : HashMap < String , StructLatticeInfo > , pub enum_lattices : HashMap < String , EnumLatticeInfo > , pub impl_lattices : HashMap < String , ImplLatticeInfo > , }
    };
}

TypeUsageVisitor!();