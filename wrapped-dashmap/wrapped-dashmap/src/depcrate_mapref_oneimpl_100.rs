// Generated macro for impl_100 (impl)
macro_rules! Depcrate_mapref_oneimpl_100 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V > RefMut < 'a , K , V > { pub (crate) fn new (guard : RwLockWriteGuardDetached < 'a > , k : & 'a K , v : & 'a mut V) -> Self { Self { guard , k , v } } pub fn key (& self) -> & K { self . pair () . 0 } pub fn value (& self) -> & V { self . pair () . 1 } pub fn value_mut (& mut self) -> & mut V { self . pair_mut () . 1 } pub fn pair (& self) -> (& K , & V) { (self . k , self . v) } pub fn pair_mut (& mut self) -> (& K , & mut V) { (self . k , self . v) } pub fn downgrade (self) -> Ref < 'a , K , V > { Ref :: new (unsafe { RwLockWriteGuardDetached :: downgrade (self . guard) } , self . k , self . v ,) } pub fn map < F , T : ? Sized > (self , f : F) -> MappedRefMut < 'a , K , T > where F : FnOnce (& mut V) -> & mut T , { MappedRefMut { _guard : self . guard , k : self . k , v : f (& mut * self . v) , } } pub fn try_map < F , T : ? Sized > (self , f : F) -> Result < MappedRefMut < 'a , K , T > , Self > where F : FnOnce (& mut V) -> Option < & mut T > , { let v = match f (unsafe { & mut * (self . v as * mut _) }) { Some (v) => v , None => return Err (self) , } ; let guard = self . guard ; let k = self . k ; Ok (MappedRefMut { _guard : guard , k , v , }) } }
};
}
