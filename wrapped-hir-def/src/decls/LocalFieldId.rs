macro_rules! deps {
    () => {
        FieldData!();
    };
}

macro_rules! LocalFieldId {
    () => {
        deps!();
        pub type LocalFieldId = Idx < FieldData > ;
    };
}

LocalFieldId!();