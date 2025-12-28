macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! TypeSubstitution {
    () => {
        deps!();
        struct TypeSubstitution < 'a > { from_name : Symbol , to_ty : & 'a ast :: Ty , rewritten : bool , }
    };
}

TypeSubstitution!()