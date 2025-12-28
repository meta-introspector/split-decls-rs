macro_rules! deps {
    () => {
        Spec!();
    };
}

macro_rules! impls {
    () => {
        deps!();
        mod impls { use std :: ops :: { Deref , DerefMut } ; use crate :: revision :: Spec ; impl Deref for Spec < '_ > { type Target = gix_revision :: Spec ; fn deref (& self) -> & Self :: Target { & self . inner } } impl DerefMut for Spec < '_ > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . inner } } impl PartialEq for Spec < '_ > { fn eq (& self , other : & Self) -> bool { self . inner == other . inner } } impl Eq for Spec < '_ > { } }
    };
}

impls!();