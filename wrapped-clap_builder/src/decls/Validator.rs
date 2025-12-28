macro_rules! deps {
    () => {
        Id!();
        Command!();
        ChildGraph!();
    };
}

macro_rules! Validator {
    () => {
        deps!();
        pub (crate) struct Validator < 'cmd > { cmd : & 'cmd Command , required : ChildGraph < Id > , }
    };
}

Validator!()