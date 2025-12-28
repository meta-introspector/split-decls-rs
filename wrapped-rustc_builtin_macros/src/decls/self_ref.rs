macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! self_ref {
    () => {
        deps!();
        pub (crate) fn self_ref () -> Ty { Ref (Box :: new (Self_) , ast :: Mutability :: Not) }
    };
}

self_ref!()