macro_rules! deps {
    () => {
        VariantDef!();
    };
}

macro_rules! Field {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Field { pub (crate) parent : VariantDef , pub (crate) id : LocalFieldId , }
    };
}

Field!();