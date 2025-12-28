macro_rules! deps {
    () => {
        MetaInputValue!();
        Registry!();
        InputValue!();
    };
}

macro_rules! __InputValue {
    () => {
        deps!();
        pub struct __InputValue < 'a > { pub registry : & 'a registry :: Registry , pub visible_types : & 'a HashSet < & 'a str > , pub input_value : & 'a registry :: MetaInputValue , }
    };
}

__InputValue!();