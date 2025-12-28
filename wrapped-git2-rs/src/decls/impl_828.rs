macro_rules! deps {
    () => {
        Error!();
        Convert!();
        ObjectType!();
        Oid!();
        TreeEntry!();
        Binding!();
        Repository!();
        Object!();
    };
}

macro_rules! impl_828 {
    () => {
        deps!();
        impl < 'tree > TreeEntry < 'tree > { # [doc = " Get the id of the object pointed by the entry"] pub fn id (& self) -> Oid { unsafe { Binding :: from_raw (raw :: git_tree_entry_id (& * self . raw)) } } # [doc = " Get the filename of a tree entry"] # [doc = ""] # [doc = " Returns `None` if the name is not valid utf-8"] pub fn name (& self) -> Option < & str > { str :: from_utf8 (self . name_bytes ()) . ok () } # [doc = " Get the filename of a tree entry"] pub fn name_bytes (& self) -> & [u8] { unsafe { crate :: opt_bytes (self , raw :: git_tree_entry_name (& * self . raw ())) . unwrap () } } # [doc = " Convert a tree entry to the object it points to."] pub fn to_object < 'a > (& self , repo : & 'a Repository) -> Result < Object < 'a > , Error > { let mut ret = ptr :: null_mut () ; unsafe { try_call ! (raw :: git_tree_entry_to_object (& mut ret , repo . raw () , &* self . raw ())) ; Ok (Binding :: from_raw (ret)) } } # [doc = " Get the type of the object pointed by the entry"] pub fn kind (& self) -> Option < ObjectType > { ObjectType :: from_raw (unsafe { raw :: git_tree_entry_type (& * self . raw) }) } # [doc = " Get the UNIX file attributes of a tree entry"] pub fn filemode (& self) -> i32 { unsafe { raw :: git_tree_entry_filemode (& * self . raw) as i32 } } # [doc = " Get the raw UNIX file attributes of a tree entry"] pub fn filemode_raw (& self) -> i32 { unsafe { raw :: git_tree_entry_filemode_raw (& * self . raw) as i32 } } # [doc = " Convert this entry of any lifetime into an owned signature with a static"] # [doc = " lifetime."] # [doc = ""] # [doc = " This will use the `Clone::clone` implementation under the hood."] pub fn to_owned (& self) -> TreeEntry < 'static > { unsafe { let me = mem :: transmute :: < & TreeEntry < 'tree > , & TreeEntry < 'static > > (self) ; me . clone () } } }
    };
}

impl_828!()