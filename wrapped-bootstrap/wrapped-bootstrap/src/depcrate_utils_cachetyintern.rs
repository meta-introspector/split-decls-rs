// Generated macro for TyIntern (struct)
macro_rules! Depcrate_utils_cacheTyIntern {
() => {
// Module: crate::utils::cache
// Provides: {"TyIntern"}
// Dependencies: {}
# [doc = " A structure for managing the interning of values of type `T`."] # [doc = ""] # [doc = " `TyIntern<T>` maintains a mapping between values and their interned representations,"] # [doc = " ensuring that duplicate values are not stored multiple times."] struct TyIntern < T : Clone + Eq > { items : Vec < T > , set : HashMap < T , Interned < T > > , }
};
}
