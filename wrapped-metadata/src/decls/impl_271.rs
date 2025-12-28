macro_rules! deps {
    () => {
        TypeName!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl TypeName { pub fn named (namespace : & str , name : & str) -> Self { TypeName { namespace : namespace . to_string () , name : name . to_string () , generics : vec ! [] , } } }
    };
}

impl_271!();