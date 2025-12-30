// Generated macro for impl_1363 (impl)
macro_rules! Depcrate_utils_cacheimpl_1363 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1363"}
// Dependencies: {}
impl < T : Hash + Clone + Eq > TyIntern < T > { # [doc = " Interns a borrowed value, ensuring it is stored uniquely."] # [doc = ""] # [doc = " If the value has been previously interned, the same `Interned<T>` instance is returned."] fn intern_borrow < B > (& mut self , item : & B) -> Interned < T > where B : Eq + Hash + ToOwned < Owned = T > + ? Sized , T : Borrow < B > , { if let Some (i) = self . set . get (item) { return * i ; } let item = item . to_owned () ; let interned = Interned (self . items . len () , PhantomData :: < * const T >) ; self . set . insert (item . clone () , interned) ; self . items . push (item) ; interned } # [doc = " Interns an owned value, storing it uniquely."] # [doc = ""] # [doc = " If the value has been previously interned, the existing `Interned<T>` is returned."] fn intern (& mut self , item : T) -> Interned < T > { if let Some (i) = self . set . get (& item) { return * i ; } let interned = Interned (self . items . len () , PhantomData :: < * const T >) ; self . set . insert (item . clone () , interned) ; self . items . push (item) ; interned } # [doc = " Retrieves a reference to the interned value associated with the given `Interned<T>` instance."] fn get (& self , i : Interned < T >) -> & T { & self . items [i . 0] } }
};
}
