macro_rules! deps {
    () => {
        Kind!();
        Version!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl Version { # [doc = " The kind of hash to produce to be compatible to this kind of index"] pub fn hash (& self) -> gix_hash :: Kind { match self { Version :: V1 | Version :: V2 => gix_hash :: Kind :: Sha1 , } } }
    };
}

impl_204!()