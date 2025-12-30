// Generated macro for into_vec (function)
macro_rules! Depcrate_serinto_vec {
() => {
// Module: crate::ser
// Provides: {"into_vec"}
// Dependencies: {}
# [cfg (feature = "std")] # [doc = " Serializes as CBOR into a new Vec<u8>"] # [inline] pub fn into_vec < T : ? Sized + ser :: Serialize > (value : & T ,) -> Result < Vec < u8 > , Error < < Vec < u8 > as ciborium_io :: Write > :: Error > > { let mut vector = vec ! [] ; let mut encoder = Serializer :: from (& mut vector) ; value . serialize (& mut encoder) ? ; Ok (vector) }
};
}
