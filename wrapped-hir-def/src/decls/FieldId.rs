macro_rules! deps {
    () => {
        VariantId!();
    };
}

macro_rules! FieldId {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct FieldId { pub parent : VariantId , pub local_id : LocalFieldId , }
    };
}

FieldId!()