macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! UnionSignature {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct UnionSignature { pub name : Name , pub generic_params : Arc < GenericParams > , pub store : Arc < ExpressionStore > , pub flags : StructFlags , pub repr : Option < ReprOptions > , }
    };
}

UnionSignature!();