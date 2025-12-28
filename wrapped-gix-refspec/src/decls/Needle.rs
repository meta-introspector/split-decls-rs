macro_rules! Needle {
    () => {
        # [derive (Debug , Copy , Clone)] pub (crate) enum Needle < 'a > { FullName (& 'a BStr) , PartialName (& 'a BStr) , Glob { name : & 'a BStr , asterisk_pos : usize } , Pattern (& 'a BStr) , Object (ObjectId) , }
    };
}

Needle!()