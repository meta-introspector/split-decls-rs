// Generated macro for impls (module)
macro_rules! Depcrate_blobimpls {
() => {
// Module: crate::blob
// Provides: {"impls"}
// Dependencies: {}
mod impls { use crate :: blob :: ResourceKind ; impl std :: fmt :: Display for ResourceKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (match self { ResourceKind :: OldOrSource => "old" , ResourceKind :: NewOrDestination => "new" , }) } } }
};
}
