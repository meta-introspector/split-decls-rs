macro_rules! deps {
    () => {
        TypeInfo!();
    };
}

macro_rules! TypeCollector {
    () => {
        deps!();
        pub struct TypeCollector < 'a > { pub type_map : & 'a mut HashMap < String , TypeInfo > , }
    };
}

TypeCollector!()