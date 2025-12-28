macro_rules! deps {
    () => {
        Storage!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl Deref for Storage { type Target = gix_hashtable :: HashMap < gix_hash :: ObjectId , (gix_object :: Kind , Vec < u8 >) > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_162!();