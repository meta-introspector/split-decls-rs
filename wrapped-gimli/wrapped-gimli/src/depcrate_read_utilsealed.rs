// Generated macro for sealed (module)
macro_rules! Depcrate_read_utilsealed {
() => {
// Module: crate::read::util
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { # [doc = " # Safety"] # [doc = " Implementer must not modify the content in storage."] pub unsafe trait Sealed { type Storage ; fn new_storage () -> Self :: Storage ; fn grow (_storage : & mut Self :: Storage , _additional : usize) -> Result < () , CapacityFull > { Err (CapacityFull) } } # [derive (Clone , Copy , Debug)] pub struct CapacityFull ; }
};
}
