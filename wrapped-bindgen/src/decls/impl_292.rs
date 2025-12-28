macro_rules! deps {
    () => {
        Dependencies!();
        CppStruct!();
        Value!();
        TypeMap!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl Dependencies for CppStruct { fn combine (& self , dependencies : & mut TypeMap) { for field in self . def . fields () { field . ty (Some (self)) . combine (dependencies) ; } if let Some (attribute) = self . def . find_attribute ("AlsoUsableForAttribute") { if let Some ((_ , Value :: Str (type_name))) = attribute . args () . first () { self . def . reader () . unwrap_full_name (self . def . namespace () , type_name) . combine (dependencies) ; } } } }
    };
}

impl_292!()