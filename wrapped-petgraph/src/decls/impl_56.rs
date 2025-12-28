macro_rules! deps {
    () => {
        WalkerIter!();
        Walker!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < W , C > WalkerIter < W , C > where W : Walker < C > , C : Clone , { pub fn context (& self) -> C { self . context . clone () } pub fn inner_ref (& self) -> & W { & self . walker } pub fn inner_mut (& mut self) -> & mut W { & mut self . walker } }
    };
}

impl_56!();