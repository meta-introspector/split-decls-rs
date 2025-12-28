macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! create_value_object {
    () => {
        deps!();
        pub (crate) fn create_value_object (values : Vec < (Name , Value) >) -> Value { let mut map = IndexMap :: new () ; for (name , value) in values { insert_value (& mut map , name , value) ; } Value :: Object (map) }
    };
}

create_value_object!()