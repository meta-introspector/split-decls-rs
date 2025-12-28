macro_rules! Label {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct Label { pub (crate) parent : DefWithBodyId , pub (crate) label_id : LabelId , }
    };
}

Label!()