// Generated macro for write (module)
macro_rules! Depcrate_objectwrite {
() => {
// Module: crate::object
// Provides: {"write"}
// Dependencies: {}
mod write { use std :: io ; use crate :: { Kind , Object , ObjectRef , WriteTo } ; # [doc = " Serialization"] impl WriteTo for ObjectRef < '_ > { # [doc = " Write the contained object to `out` in the git serialization format."] fn write_to (& self , out : & mut dyn io :: Write) -> io :: Result < () > { use crate :: ObjectRef :: * ; match self { Tree (v) => v . write_to (out) , Blob (v) => v . write_to (out) , Commit (v) => v . write_to (out) , Tag (v) => v . write_to (out) , } } fn kind (& self) -> Kind { self . kind () } fn size (& self) -> u64 { use crate :: ObjectRef :: * ; match self { Tree (v) => v . size () , Blob (v) => v . size () , Commit (v) => v . size () , Tag (v) => v . size () , } } } # [doc = " Serialization"] impl WriteTo for Object { # [doc = " Write the contained object to `out` in the git serialization format."] fn write_to (& self , out : & mut dyn io :: Write) -> io :: Result < () > { use crate :: Object :: * ; match self { Tree (v) => v . write_to (out) , Blob (v) => v . write_to (out) , Commit (v) => v . write_to (out) , Tag (v) => v . write_to (out) , } } fn kind (& self) -> Kind { self . kind () } fn size (& self) -> u64 { use crate :: Object :: * ; match self { Tree (v) => v . size () , Blob (v) => v . size () , Commit (v) => v . size () , Tag (v) => v . size () , } } } }
};
}
