// Generated macro for impl_115 (impl)
macro_rules! Depcrate_objectimpl_115 {
() => {
// Module: crate::object
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'a > ObjectRef < 'a > { # [doc = " Deserialize an object from a loose serialisation"] pub fn from_loose (data : & 'a [u8]) -> Result < ObjectRef < 'a > , LooseDecodeError > { let (kind , size , offset) = loose_header (data) ? ; let body = & data [offset ..] . get (.. size . try_into () . map_err (| _ | LooseDecodeError :: OutOfMemory { size }) ?) . ok_or (LooseHeaderDecodeError :: InvalidHeader { message : "object data was shorter than its size declared in the header" , }) ? ; Ok (Self :: from_bytes (kind , body) ?) } # [doc = " Deserialize an object of `kind` from the given `data`."] pub fn from_bytes (kind : Kind , data : & 'a [u8]) -> Result < ObjectRef < 'a > , crate :: decode :: Error > { Ok (match kind { Kind :: Tree => ObjectRef :: Tree (TreeRef :: from_bytes (data) ?) , Kind :: Blob => ObjectRef :: Blob (BlobRef { data }) , Kind :: Commit => ObjectRef :: Commit (CommitRef :: from_bytes (data) ?) , Kind :: Tag => ObjectRef :: Tag (TagRef :: from_bytes (data) ?) , }) } # [doc = " Convert the immutable object into a mutable version, consuming the source in the process."] # [doc = ""] # [doc = " Note that this is an expensive operation."] pub fn into_owned (self) -> Result < Object , crate :: decode :: Error > { self . try_into () } # [doc = " Convert this immutable object into its mutable counterpart."] # [doc = ""] # [doc = " Note that this is an expensive operation."] pub fn to_owned (& self) -> Result < Object , crate :: decode :: Error > { self . clone () . try_into () } }
};
}
