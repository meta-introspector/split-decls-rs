// Generated macro for into_writer (function)
macro_rules! Depcrate_serinto_writer {
() => {
// Module: crate::ser
// Provides: {"into_writer"}
// Dependencies: {}
# [doc = " Serializes as CBOR into a type with [`impl ciborium_io::Write`](ciborium_io::Write)"] # [inline] pub fn into_writer < T : ? Sized + ser :: Serialize , W : Write > (value : & T , writer : W ,) -> Result < () , Error < W :: Error > > where W :: Error : core :: fmt :: Debug , { let mut encoder = Serializer :: from (writer) ; value . serialize (& mut encoder) }
};
}
