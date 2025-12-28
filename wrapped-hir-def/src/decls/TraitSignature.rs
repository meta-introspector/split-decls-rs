macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! TraitSignature {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct TraitSignature { pub name : Name , pub generic_params : Arc < GenericParams > , pub store : Arc < ExpressionStore > , pub flags : TraitFlags , }
    };
}

TraitSignature!()