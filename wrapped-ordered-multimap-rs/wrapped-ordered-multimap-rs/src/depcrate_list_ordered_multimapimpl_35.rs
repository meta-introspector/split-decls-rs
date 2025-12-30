// Generated macro for impl_35 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_35 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_35"}
// Dependencies: {}
impl < Key , Value , State > Debug for Entry < '_ , Key , Value , State > where Key : Debug , State : BuildHasher , Value : Debug , { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { match self { Entry :: Occupied (entry) => entry . fmt (formatter) , Entry :: Vacant (entry) => entry . fmt (formatter) , } } }
};
}
