macro_rules! deps {
    () => {
        Command!();
        ChildGraph!();
        Id!();
    };
}

macro_rules! Validator {
    () => {
        deps!();
        pub (crate) struct Validator < 'cmd > { cmd : & 'cmd Command , required : ChildGraph < Id > , }
    };
}

Validator!();