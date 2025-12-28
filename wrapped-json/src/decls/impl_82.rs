macro_rules! deps {
    () => {
        Map!();
        Value!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl PartialEq for Map < String , Value > { # [inline] fn eq (& self , other : & Self) -> bool { self . map . eq (& other . map) } }
    };
}

impl_82!();