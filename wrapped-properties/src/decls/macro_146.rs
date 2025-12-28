macro_rules! deps {
    () => {
        JoiningType!();
    };
}

macro_rules! macro_146 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Joining_Type" ; short_name : "jt" ; ident : JoiningType ; data_marker : crate :: provider :: PropertyEnumJoiningTypeV1 ; singleton : SINGLETON_PROPERTY_ENUM_JOINING_TYPE_V1 ; ule_ty : u8 ; }
    };
}

macro_146!()