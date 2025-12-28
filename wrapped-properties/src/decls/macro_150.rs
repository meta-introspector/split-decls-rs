macro_rules! deps {
    () => {
        VerticalOrientation!();
    };
}

macro_rules! macro_150 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Vertical_Orientation" ; short_name : "vo" ; ident : VerticalOrientation ; data_marker : crate :: provider :: PropertyEnumVerticalOrientationV1 ; singleton : SINGLETON_PROPERTY_ENUM_VERTICAL_ORIENTATION_V1 ; ule_ty : u8 ; }
    };
}

macro_150!()