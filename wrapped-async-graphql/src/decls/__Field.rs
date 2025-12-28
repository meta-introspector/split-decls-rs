macro_rules! deps {
    () => {
        Field!();
        Registry!();
        MetaField!();
    };
}

macro_rules! __Field {
    () => {
        deps!();
        pub struct __Field < 'a > { pub registry : & 'a registry :: Registry , pub visible_types : & 'a HashSet < & 'a str > , pub field : & 'a registry :: MetaField , }
    };
}

__Field!();