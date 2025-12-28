macro_rules! deps {
    () => {
        AsName!();
        Name!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl AsName for ast :: NameRef { fn as_name (& self) -> Name { match self . as_tuple_field () { Some (idx) => Name :: new_tuple_field (idx) , None => Name :: new_root (& self . text ()) , } } }
    };
}

impl_151!()