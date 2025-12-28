macro_rules! deps {
    () => {
        Kind!();
        Tree!();
        Error!();
        Repository!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl gix_object :: Find for crate :: Repository { fn try_find < 'a > (& self , id : & gix_hash :: oid , buffer : & 'a mut Vec < u8 > ,) -> Result < Option < gix_object :: Data < 'a > > , gix_object :: find :: Error > { if id == ObjectId :: empty_tree (self . object_hash ()) { buffer . clear () ; return Ok (Some (gix_object :: Data { kind : gix_object :: Kind :: Tree , data : & [] , })) ; } self . objects . try_find (id , buffer) } }
    };
}

impl_337!()