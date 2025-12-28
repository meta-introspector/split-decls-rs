macro_rules! deps {
    () => {
        Script!();
    };
}

macro_rules! macro_106 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Script" ; short_name : "sc" ; ident : Script ; data_marker : crate :: provider :: PropertyEnumScriptV1 ; singleton : SINGLETON_PROPERTY_ENUM_SCRIPT_V1 ; ule_ty : < u16 as zerovec :: ule :: AsULE >:: ULE ; }
    };
}

macro_106!()