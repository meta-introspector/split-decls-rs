macro_rules! deps {
    () => {
        TupleField!();
        Type!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl TupleField { pub fn name (& self) -> Name { Name :: new_tuple_field (self . index as usize) } pub fn ty < 'db > (& self , db : & 'db dyn HirDatabase) -> Type < 'db > { let interner = DbInterner :: new_with (db , None , None) ; let ty = db . infer (self . owner) . tuple_field_access_type (self . tuple) . as_slice () . get (self . index as usize) . copied () . unwrap_or_else (| | Ty :: new_error (interner , ErrorGuaranteed)) ; Type { env : db . trait_environment_for_body (self . owner) , ty } } }
    };
}

impl_248!()