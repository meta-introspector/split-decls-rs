macro_rules! OutputType {
    () => {
        pub enum OutputType < 'a > { Value (& 'a Type) , Result (& 'a Type) , }
    };
}

OutputType!();