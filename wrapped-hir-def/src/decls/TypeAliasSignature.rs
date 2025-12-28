macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! TypeAliasSignature {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct TypeAliasSignature { pub name : Name , pub generic_params : Arc < GenericParams > , pub store : Arc < ExpressionStore > , pub bounds : Box < [TypeBound] > , pub ty : Option < TypeRefId > , pub flags : TypeAliasFlags , }
    };
}

TypeAliasSignature!()