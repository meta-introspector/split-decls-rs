macro_rules! deps {
    () => {
        Struct!();
        Variant!();
        Union!();
    };
}

macro_rules! VariantDef {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum VariantDef { Struct (Struct) , Union (Union) , Variant (Variant) , }
    };
}

VariantDef!()