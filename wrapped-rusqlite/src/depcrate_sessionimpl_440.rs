// Generated macro for impl_440 (impl)
macro_rules! Depcrate_sessionimpl_440 {
() => {
// Module: crate::session
// Provides: {"impl_440"}
// Dependencies: {}
impl Changeset { # [doc = " Invert a changeset"] # [inline] pub fn invert (& self) -> Result < Changeset > { let mut n = 0 ; let mut cs = ptr :: null_mut () ; check (unsafe { ffi :: sqlite3changeset_invert (self . n , self . cs , & mut n , & mut cs as * mut * mut _) }) ? ; Ok (Changeset { cs , n }) } # [doc = " Create an iterator to traverse a changeset"] # [inline] pub fn iter (& self) -> Result < ChangesetIter < '_ > > { let mut it = ptr :: null_mut () ; check (unsafe { ffi :: sqlite3changeset_start (& mut it as * mut * mut _ , self . n , self . cs) }) ? ; Ok (ChangesetIter { phantom : PhantomData , it , item : None , }) } # [doc = " Concatenate two changeset objects"] # [inline] pub fn concat (a : & Changeset , b : & Changeset) -> Result < Changeset > { let mut n = 0 ; let mut cs = ptr :: null_mut () ; check (unsafe { ffi :: sqlite3changeset_concat (a . n , a . cs , b . n , b . cs , & mut n , & mut cs as * mut * mut _) }) ? ; Ok (Changeset { cs , n }) } }
};
}
