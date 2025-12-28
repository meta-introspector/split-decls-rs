macro_rules! deps {
    () => {
        Kind!();
        Object!();
        Never!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl cache :: Object for Never { # [doc = " Noop"] fn put (& mut self , _id : gix_hash :: ObjectId , _kind : gix_object :: Kind , _data : & [u8]) { } # [doc = " Noop"] fn get (& mut self , _id : & gix_hash :: ObjectId , _out : & mut Vec < u8 >) -> Option < gix_object :: Kind > { None } }
    };
}

impl_42!();