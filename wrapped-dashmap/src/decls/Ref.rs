macro_rules! Ref {
    () => {
        pub struct Ref < 'a , K > { inner : mapref :: one :: Ref < 'a , K , () > , }
    };
}

Ref!();