macro_rules! deps {
    () => {
        Type!();
        CppStruct!();
    };
}

macro_rules! is_signed_error {
    () => {
        deps!();
        fn is_signed_error (ty : & Type) -> bool { match ty { Type :: HRESULT => true , Type :: CppStruct (ty) => ! ty . def . underlying_type () . is_unsigned () , _ => false , } }
    };
}

is_signed_error!()