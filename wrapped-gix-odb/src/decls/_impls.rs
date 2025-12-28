macro_rules! deps {
    () => {
        Error!();
        Header!();
    };
}

macro_rules! _impls {
    () => {
        deps!();
        mod _impls { use std :: { ops :: Deref , rc :: Rc , sync :: Arc } ; use gix_hash :: oid ; use crate :: find :: Header ; impl < T > crate :: Header for & T where T : crate :: Header , { fn try_header (& self , id : & oid) -> Result < Option < Header > , gix_object :: find :: Error > { (* self) . try_header (id) } } impl < T > crate :: Header for Rc < T > where T : crate :: Header , { fn try_header (& self , id : & oid) -> Result < Option < Header > , gix_object :: find :: Error > { self . deref () . try_header (id) } } impl < T > crate :: Header for Arc < T > where T : crate :: Header , { fn try_header (& self , id : & oid) -> Result < Option < Header > , gix_object :: find :: Error > { self . deref () . try_header (id) } } }
    };
}

_impls!();