// Generated macro for impl_359 (impl)
macro_rules! Depcrateimpl_359 {
() => {
// Module: crate
// Provides: {"impl_359"}
// Dependencies: {}
impl < 'db > InstantiatedEnum < 'db > { pub fn ty (self , db : & 'db dyn HirDatabase) -> TypeNs < 'db > { let krate = self . inner . krate (db) ; let interner = DbInterner :: new_with (db , Some (krate . base ()) , None) ; let ty = db . ty (self . inner . id . into ()) ; TypeNs :: new (db , self . inner . id , ty . instantiate (interner , self . args)) } }
};
}
