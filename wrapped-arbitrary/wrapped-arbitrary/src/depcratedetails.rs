// Generated macro for details (module)
macro_rules! Depcratedetails {
() => {
// Module: crate
// Provides: {"details"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "derive")] pub mod details { use super :: * ; pub trait IsEmpty { fn is_empty (& self) -> bool ; } impl IsEmpty for Unstructured < '_ > { fn is_empty (& self) -> bool { Unstructured :: is_empty (self) } } impl IsEmpty for & mut Unstructured < '_ > { fn is_empty (& self) -> bool { Unstructured :: is_empty (self) } } # [inline] pub fn with_recursive_count < U : IsEmpty , R > (u : U , recursive_count : & 'static std :: thread :: LocalKey < std :: cell :: Cell < u32 > > , f : impl FnOnce (U) -> Result < R > ,) -> Result < R > { let guard_against_recursion = u . is_empty () ; if guard_against_recursion { recursive_count . with (| count | { if count . get () > 0 { return Err (Error :: NotEnoughData) ; } count . set (count . get () + 1) ; Ok (()) }) ? ; } let result = f (u) ; if guard_against_recursion { recursive_count . with (| count | { count . set (count . get () - 1) ; }) ; } result } }
};
}
