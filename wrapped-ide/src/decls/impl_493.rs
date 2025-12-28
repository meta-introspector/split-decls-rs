macro_rules! deps {
    () => {
        FieldOrTupleIdx!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl FieldOrTupleIdx { fn name (& self , db : & RootDatabase) -> String { match * self { FieldOrTupleIdx :: Field (f) => f . name (db) . as_str () . to_owned () , FieldOrTupleIdx :: TupleIdx (i) => format ! (".{i}") , } } }
    };
}

impl_493!()