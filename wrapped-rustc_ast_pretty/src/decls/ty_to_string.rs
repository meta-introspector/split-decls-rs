macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! ty_to_string {
    () => {
        deps!();
        pub fn ty_to_string (ty : & ast :: Ty) -> String { State :: new () . ty_to_string (ty) }
    };
}

ty_to_string!()