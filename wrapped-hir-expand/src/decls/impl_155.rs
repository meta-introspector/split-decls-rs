macro_rules! deps {
    () => {
        Name!();
        AsName!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl AsName for ast :: FieldKind { fn as_name (& self) -> Name { match self { ast :: FieldKind :: Name (nr) => nr . as_name () , ast :: FieldKind :: Index (idx) => { let idx = idx . text () . parse :: < usize > () . unwrap_or (0) ; Name :: new_tuple_field (idx) } } } }
    };
}

impl_155!()