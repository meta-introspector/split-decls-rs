macro_rules! deps {
    () => {
        FieldValueInner!();
    };
}

macro_rules! FieldValue {
    () => {
        deps!();
        # [doc = " A value returned from the resolver function"] pub struct FieldValue < 'a > (pub (crate) FieldValueInner < 'a >) ;
    };
}

FieldValue!()