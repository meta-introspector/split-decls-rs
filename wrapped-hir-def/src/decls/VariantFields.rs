macro_rules! deps {
    () => {
        FieldsShape!();
        FieldData!();
        ExpressionStore!();
    };
}

macro_rules! VariantFields {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct VariantFields { fields : Arena < FieldData > , pub store : Arc < ExpressionStore > , pub shape : FieldsShape , }
    };
}

VariantFields!()