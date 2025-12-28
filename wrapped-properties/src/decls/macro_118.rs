macro_rules! deps {
    () => {
        LineBreak!();
    };
}

macro_rules! macro_118 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Line_Break" ; short_name : "lb" ; ident : LineBreak ; data_marker : crate :: provider :: PropertyEnumLineBreakV1 ; singleton : SINGLETON_PROPERTY_ENUM_LINE_BREAK_V1 ; ule_ty : u8 ; }
    };
}

macro_118!();