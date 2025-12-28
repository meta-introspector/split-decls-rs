macro_rules! deps {
    () => {
        ExpressionStore!();
        ExprId!();
    };
}

macro_rules! VariantFieldsBody {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct VariantFieldsBody { pub store : Arc < ExpressionStore > , pub fields : Box < [Option < ExprId >] > , }
    };
}

VariantFieldsBody!();