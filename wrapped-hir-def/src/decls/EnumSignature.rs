macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! EnumSignature {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct EnumSignature { pub name : Name , pub generic_params : Arc < GenericParams > , pub store : Arc < ExpressionStore > , pub flags : EnumFlags , pub repr : Option < ReprOptions > , }
    };
}

EnumSignature!()