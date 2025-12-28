macro_rules! deps {
    () => {
        FieldsShape!();
    };
}

macro_rules! EnumVariants {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct EnumVariants { pub variants : Box < [(EnumVariantId , Name , FieldsShape)] > , }
    };
}

EnumVariants!()