macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! ConstSignature {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct ConstSignature { pub name : Option < Name > , pub store : Arc < ExpressionStore > , pub type_ref : TypeRefId , pub flags : ConstFlags , }
    };
}

ConstSignature!()