macro_rules! deps {
    () => {
        Header!();
    };
}

macro_rules! header {
    () => {
        deps!();
        mod header { use super :: Header ; impl Header { # [doc = " Return the object kind of the object we represent."] pub fn kind (& self) -> gix_object :: Kind { match self { Header :: Packed (out) => out . kind , Header :: Loose { kind , .. } => * kind , } } # [doc = " Return the size of the object in bytes."] pub fn size (& self) -> u64 { match self { Header :: Packed (out) => out . object_size , Header :: Loose { size , .. } => * size , } } # [doc = " Return the amount of deltas decoded to obtain this header, if the object was packed."] pub fn num_deltas (& self) -> Option < u32 > { match self { Header :: Packed (out) => out . num_deltas . into () , Header :: Loose { .. } => None , } } } impl From < gix_pack :: data :: decode :: header :: Outcome > for Header { fn from (packed_header : gix_pack :: data :: decode :: header :: Outcome) -> Self { Header :: Packed (packed_header) } } impl From < (u64 , gix_object :: Kind) > for Header { fn from ((object_size , kind) : (u64 , gix_object :: Kind)) -> Self { Header :: Loose { kind , size : object_size , } } } }
    };
}

header!()