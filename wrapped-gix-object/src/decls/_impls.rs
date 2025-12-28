macro_rules! deps {
    () => {
        Data!();
        Find!();
        Error!();
        FindObjectOrHeader!();
        Exists!();
        Header!();
    };
}

macro_rules! _impls {
    () => {
        deps!();
        mod _impls { use std :: { ops :: Deref , rc :: Rc , sync :: Arc } ; use gix_hash :: oid ; use crate :: Data ; impl < T > crate :: Exists for & T where T : crate :: Exists , { fn exists (& self , id : & oid) -> bool { (* self) . exists (id) } } impl < T > crate :: FindObjectOrHeader for T where T : crate :: Find + crate :: FindHeader { } impl < T > crate :: Find for & T where T : crate :: Find , { fn try_find < 'a > (& self , id : & oid , buffer : & 'a mut Vec < u8 >) -> Result < Option < Data < 'a > > , crate :: find :: Error > { (* self) . try_find (id , buffer) } } impl < T > crate :: FindHeader for & T where T : crate :: FindHeader , { fn try_header (& self , id : & gix_hash :: oid) -> Result < Option < crate :: Header > , crate :: find :: Error > { (* self) . try_header (id) } } impl < T > crate :: Exists for Box < T > where T : crate :: Exists , { fn exists (& self , id : & oid) -> bool { self . deref () . exists (id) } } impl < T > crate :: Exists for Rc < T > where T : crate :: Exists , { fn exists (& self , id : & oid) -> bool { self . deref () . exists (id) } } impl < T > crate :: Find for Rc < T > where T : crate :: Find , { fn try_find < 'a > (& self , id : & oid , buffer : & 'a mut Vec < u8 >) -> Result < Option < Data < 'a > > , crate :: find :: Error > { self . deref () . try_find (id , buffer) } } impl < T > crate :: FindHeader for Rc < T > where T : crate :: FindHeader , { fn try_header (& self , id : & gix_hash :: oid) -> Result < Option < crate :: Header > , crate :: find :: Error > { self . deref () . try_header (id) } } impl < T > crate :: Find for Box < T > where T : crate :: Find , { fn try_find < 'a > (& self , id : & oid , buffer : & 'a mut Vec < u8 >) -> Result < Option < Data < 'a > > , crate :: find :: Error > { self . deref () . try_find (id , buffer) } } impl < T > crate :: FindHeader for Box < T > where T : crate :: FindHeader , { fn try_header (& self , id : & gix_hash :: oid) -> Result < Option < crate :: Header > , crate :: find :: Error > { self . deref () . try_header (id) } } impl < T > crate :: Exists for Arc < T > where T : crate :: Exists , { fn exists (& self , id : & oid) -> bool { self . deref () . exists (id) } } impl < T > crate :: Find for Arc < T > where T : crate :: Find , { fn try_find < 'a > (& self , id : & oid , buffer : & 'a mut Vec < u8 >) -> Result < Option < Data < 'a > > , crate :: find :: Error > { self . deref () . try_find (id , buffer) } } impl < T > crate :: FindHeader for Arc < T > where T : crate :: FindHeader , { fn try_header (& self , id : & gix_hash :: oid) -> Result < Option < crate :: Header > , crate :: find :: Error > { self . deref () . try_header (id) } } }
    };
}

_impls!()