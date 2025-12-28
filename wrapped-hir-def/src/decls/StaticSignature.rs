macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! StaticSignature {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct StaticSignature { pub name : Name , pub store : Arc < ExpressionStore > , pub type_ref : TypeRefId , pub flags : StaticFlags , }
    };
}

StaticSignature!()