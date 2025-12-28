macro_rules! Variant {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Variant { pub (crate) id : EnumVariantId , }
    };
}

Variant!()