macro_rules! deps {
    () => {
        FieldData!();
        FieldsShape!();
        ExpressionStore!();
    };
}

macro_rules! VariantFields {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct VariantFields { fields : Arena < FieldData > , pub store : Arc < ExpressionStore > , pub shape : FieldsShape , }
    };
}

VariantFields!();