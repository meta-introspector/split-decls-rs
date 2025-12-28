macro_rules! deps {
    () => {
        Kind!();
        ObjectId!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl From < & oid > for ObjectId { fn from (v : & oid) -> Self { match v . kind () { Kind :: Sha1 => ObjectId :: from_20_bytes (v . as_bytes ()) , } } }
    };
}

impl_16!()