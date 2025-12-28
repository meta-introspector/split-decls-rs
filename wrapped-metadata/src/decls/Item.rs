macro_rules! deps {
    () => {
        Type!();
        Field!();
        TypeDef!();
        MethodDef!();
    };
}

macro_rules! Item {
    () => {
        deps!();
        pub enum Item < 'a > { Type (TypeDef < 'a >) , Fn (MethodDef < 'a >) , Const (Field < 'a >) , }
    };
}

Item!();