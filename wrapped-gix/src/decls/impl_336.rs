macro_rules! deps {
    () => {
        Tree!();
        Error!();
        Repository!();
        Kind!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl gix_object :: FindHeader for crate :: Repository { fn try_header (& self , id : & gix_hash :: oid) -> Result < Option < gix_object :: Header > , gix_object :: find :: Error > { if id == ObjectId :: empty_tree (self . object_hash ()) { return Ok (Some (gix_object :: Header { kind : gix_object :: Kind :: Tree , size : 0 , })) ; } self . objects . try_header (id) } }
    };
}

impl_336!();