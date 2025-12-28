macro_rules! deps {
    () => {
        TypeMap!();
        Dependencies!();
        CppFn!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl Dependencies for CppFn { fn combine (& self , dependencies : & mut TypeMap) { self . method . signature (self . namespace , & []) . combine (dependencies) ; let dependency = match self . method . name () { "GetWindowLongPtrA" => Some ("GetWindowLongA") , "GetWindowLongPtrW" => Some ("GetWindowLongW") , "SetWindowLongPtrA" => Some ("SetWindowLongA") , "SetWindowLongPtrW" => Some ("SetWindowLongW") , _ => None , } ; if let Some (dependency) = dependency { self . method . reader () . unwrap_full_name (self . namespace , dependency) . combine (dependencies) ; } } }
    };
}

impl_264!();