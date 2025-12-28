macro_rules! deps {
    () => {
        Schema!();
        Registry!();
    };
}

macro_rules! __Schema {
    () => {
        deps!();
        pub struct __Schema < 'a > { registry : & 'a registry :: Registry , visible_types : & 'a HashSet < & 'a str > , }
    };
}

__Schema!()