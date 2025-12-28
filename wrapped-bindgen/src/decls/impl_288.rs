macro_rules! deps {
    () => {
        CppStruct!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl PartialEq for CppStruct { fn eq (& self , other : & Self) -> bool { self . def == other . def } }
    };
}

impl_288!();