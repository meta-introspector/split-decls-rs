macro_rules! deps {
    () => {
        Name!();
        AsName!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl AsName for ast :: NameOrNameRef { fn as_name (& self) -> Name { match self { ast :: NameOrNameRef :: Name (it) => it . as_name () , ast :: NameOrNameRef :: NameRef (it) => it . as_name () , } } }
    };
}

impl_153!()