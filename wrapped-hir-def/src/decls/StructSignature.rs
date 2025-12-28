macro_rules! deps {
    () => {
        FieldsShape!();
        ExpressionStore!();
    };
}

macro_rules! StructSignature {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct StructSignature { pub name : Name , pub generic_params : Arc < GenericParams > , pub store : Arc < ExpressionStore > , pub flags : StructFlags , pub shape : FieldsShape , pub repr : Option < ReprOptions > , }
    };
}

StructSignature!();