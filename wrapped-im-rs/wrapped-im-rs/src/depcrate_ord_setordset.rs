// Generated macro for ordset (macro)
macro_rules! Depcrate_ord_setordset {
() => {
// Module: crate::ord::set
// Provides: {"ordset"}
// Dependencies: {}
# [doc = " Construct a set from a sequence of values."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate im;"] # [doc = " # use im::ordset::OrdSet;"] # [doc = " # fn main() {"] # [doc = " assert_eq!("] # [doc = "   ordset![1, 2, 3],"] # [doc = "   OrdSet::from(vec![1, 2, 3])"] # [doc = " );"] # [doc = " # }"] # [doc = " ```"] # [macro_export] macro_rules ! ordset { () => { $ crate :: ordset :: OrdSet :: new () } ; ($ ($ x : expr) ,*) => { { let mut l = $ crate :: ordset :: OrdSet :: new () ; $ (l . insert ($ x) ;) * l } } ; }
};
}
