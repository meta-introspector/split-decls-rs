macro_rules! deps {
    () => {
        SenderFlavor!();
        Sender!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T > Clone for Sender < T > { fn clone (& self) -> Self { let flavor = match & self . flavor { SenderFlavor :: Array (chan) => SenderFlavor :: Array (chan . acquire ()) , SenderFlavor :: List (chan) => SenderFlavor :: List (chan . acquire ()) , SenderFlavor :: Zero (chan) => SenderFlavor :: Zero (chan . acquire ()) , } ; Self { flavor } } }
    };
}

impl_15!();