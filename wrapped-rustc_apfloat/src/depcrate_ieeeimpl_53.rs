// Generated macro for impl_53 (impl)
macro_rules! Depcrate_ieeeimpl_53 {
() => {
// Module: crate::ieee
// Provides: {"impl_53"}
// Dependencies: {}
impl < S : Semantics > PartialOrd for IeeeFloat < S > { fn partial_cmp (& self , rhs : & Self) -> Option < Ordering > { match (self . category () , rhs . category ()) { (Category :: NaN , _) | (_ , Category :: NaN) => None , (Category :: Infinity , Category :: Infinity) => Some ((! self . is_negative ()) . cmp (& (! rhs . is_negative ()))) , (Category :: Zero , Category :: Zero) => Some (Ordering :: Equal) , (Category :: Infinity , _) | (Category :: Normal , Category :: Zero) => { Some ((! self . is_negative ()) . cmp (& self . is_negative ())) } (_ , Category :: Infinity) | (Category :: Zero , Category :: Normal) => { Some (rhs . is_negative () . cmp (& (! rhs . is_negative ()))) } (Category :: Normal , Category :: Normal) => { Some ((! self . is_negative ()) . cmp (& (! rhs . is_negative ())) . then_with (| | { let result = self . cmp_abs_normal (* rhs) ; if self . is_negative () { result . reverse () } else { result } })) } } } }
};
}
