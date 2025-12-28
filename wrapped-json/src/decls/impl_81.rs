macro_rules! deps {
    () => {
        Value!();
        Map!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl Clone for Map < String , Value > { # [inline] fn clone (& self) -> Self { Map { map : self . map . clone () , } } # [inline] fn clone_from (& mut self , source : & Self) { self . map . clone_from (& source . map) ; } }
    };
}

impl_81!();