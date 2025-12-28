macro_rules! deps {
    () => {
        TypeName!();
    };
}

macro_rules! Derive {
    () => {
        deps!();
        pub struct Derive (HashMap < TypeName , Vec < String > >) ;
    };
}

Derive!();