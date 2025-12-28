macro_rules! deps {
    () => {
        TypeDef!();
        MethodDef!();
        Field!();
        Type!();
    };
}

macro_rules! Item {
    () => {
        deps!();
        pub enum Item < 'a > { Type (TypeDef < 'a >) , Fn (MethodDef < 'a >) , Const (Field < 'a >) , }
    };
}

Item!()