macro_rules! FieldsShape {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum FieldsShape { Record , Tuple , Unit , }
    };
}

FieldsShape!();