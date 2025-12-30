// Generated macro for impl_482 (impl)
macro_rules! Depcrate_contextimpl_482 {
() => {
// Module: crate::context
// Provides: {"impl_482"}
// Dependencies: {}
impl < 'a > QueryPathNode < 'a > { # [doc = " Get the current field name."] # [doc = ""] # [doc = " This traverses all the parents of the node until it finds one that is a"] # [doc = " field name."] pub fn field_name (& self) -> & str { std :: iter :: once (self) . chain (self . parents ()) . find_map (| node | match node . segment { QueryPathSegment :: Name (name) => Some (name) , QueryPathSegment :: Index (_) => None , }) . unwrap () } # [doc = " Get the path represented by `Vec<String>`; numbers will be stringified."] # [must_use] pub fn to_string_vec (self) -> Vec < String > { let mut res = Vec :: new () ; self . for_each (| s | { res . push (match s { QueryPathSegment :: Name (name) => (* name) . to_string () , QueryPathSegment :: Index (idx) => idx . to_string () , }) ; }) ; res } # [doc = " Iterate over the parents of the node."] pub fn parents (& self) -> Parents < '_ > { Parents (self) } pub (crate) fn for_each < F : FnMut (& QueryPathSegment < 'a >) > (& self , mut f : F) { let _ = self . try_for_each :: < std :: convert :: Infallible , _ > (| segment | { f (segment) ; Ok (()) }) ; } pub (crate) fn try_for_each < E , F : FnMut (& QueryPathSegment < 'a >) -> Result < () , E > > (& self , mut f : F ,) -> Result < () , E > { self . try_for_each_ref (& mut f) } fn try_for_each_ref < E , F : FnMut (& QueryPathSegment < 'a >) -> Result < () , E > > (& self , f : & mut F ,) -> Result < () , E > { if let Some (parent) = & self . parent { parent . try_for_each_ref (f) ? ; } f (& self . segment) } }
};
}
