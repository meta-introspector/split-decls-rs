macro_rules! deps {
    () => {
        Value!();
        Map!();
    };
}

macro_rules! SerializeStructVariant {
    () => {
        deps!();
        pub struct SerializeStructVariant { name : String , map : Map < String , Value > , }
    };
}

SerializeStructVariant!()