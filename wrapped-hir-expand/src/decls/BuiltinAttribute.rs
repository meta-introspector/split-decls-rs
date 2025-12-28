macro_rules! deps {
    () => {
        AttributeTemplate!();
    };
}

macro_rules! BuiltinAttribute {
    () => {
        deps!();
        pub struct BuiltinAttribute { pub name : & 'static str , pub template : AttributeTemplate , }
    };
}

BuiltinAttribute!()