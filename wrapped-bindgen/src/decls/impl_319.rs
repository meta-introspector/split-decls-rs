macro_rules! deps {
    () => {
        CppDelegate!();
        CppFn!();
        Type!();
        CppEnum!();
        CppConst!();
        Class!();
        TypeMap!();
        Delegate!();
        GUID!();
        Dependencies!();
        CppStruct!();
        Interface!();
        CppInterface!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl Dependencies for Type { fn combine (& self , dependencies : & mut TypeMap) { let ty = self . decay () ; if ty . is_intrinsic () { return ; } let mut nested = false ; if let Self :: CppStruct (ty) = ty { if ty . def . namespace () . is_empty () { nested = true ; } } let (ty , generics) = ty . split_generic () ; for ty in generics { ty . combine (dependencies) ; } if ! nested && ! dependencies . insert (ty . clone ()) { return ; } if let Some (multi) = match & ty { Self :: CppStruct (ty) => Some (ty . def . reader () . with_full_name (ty . def . namespace () , ty . def . name ()) ,) , Self :: CppFn (ty) => Some (ty . method . reader () . with_full_name (ty . namespace , ty . method . name ()) ,) , _ => None , } { multi . for_each (| multi | { if ty != multi { multi . combine (dependencies) } }) ; } match & ty { Self :: Class (ty) => ty . combine (dependencies) , Self :: Delegate (ty) => ty . combine (dependencies) , Self :: Enum (..) => { } Self :: Interface (ty) => ty . combine (dependencies) , Self :: Struct (ty) => ty . combine (dependencies) , Self :: CppConst (ty) => ty . combine (dependencies) , Self :: CppDelegate (ty) => ty . combine (dependencies) , Self :: CppFn (ty) => ty . combine (dependencies) , Self :: CppInterface (ty) => ty . combine (dependencies) , Self :: CppStruct (ty) => ty . combine (dependencies) , Self :: CppEnum (ty) => ty . combine (dependencies) , Self :: IUnknown => { Self :: GUID . combine (dependencies) ; Self :: HRESULT . combine (dependencies) ; } Self :: Object => { Self :: IUnknown . combine (dependencies) ; } _ => { } } } }
    };
}

impl_319!();