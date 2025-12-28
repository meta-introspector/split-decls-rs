macro_rules! deps {
    () => {
        Union!();
        Struct!();
        Variant!();
    };
}

macro_rules! VariantDef {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum VariantDef { Struct (Struct) , Union (Union) , Variant (Variant) , }
    };
}

VariantDef!()