macro_rules! deps {
    () => {
        TypeMap!();
        Dependencies!();
        Value!();
        CppEnum!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl Dependencies for CppEnum { fn combine (& self , dependencies : & mut TypeMap) { if let Some (attribute) = self . def . find_attribute ("AlsoUsableForAttribute") { if let Some ((_ , Value :: Str (type_name))) = attribute . args () . first () { self . def . reader () . unwrap_full_name (self . def . namespace () , type_name) . combine (dependencies) ; } } } }
    };
}

impl_258!()