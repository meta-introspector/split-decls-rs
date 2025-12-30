// Generated macro for impl_122 (impl)
macro_rules! Depcrate_utility_typesimpl_122 {
() => {
// Module: crate::utility_types
// Provides: {"impl_122"}
// Dependencies: {}
impl < T > TokenAtOffset < T > { pub fn map < F : Fn (T) -> U , U > (self , f : F) -> TokenAtOffset < U > { match self { TokenAtOffset :: None => TokenAtOffset :: None , TokenAtOffset :: Single (it) => TokenAtOffset :: Single (f (it)) , TokenAtOffset :: Between (l , r) => TokenAtOffset :: Between (f (l) , f (r)) , } } # [doc = " Convert to option, preferring the right leaf in case of a tie."] pub fn right_biased (self) -> Option < T > { match self { TokenAtOffset :: None => None , TokenAtOffset :: Single (node) => Some (node) , TokenAtOffset :: Between (_ , right) => Some (right) , } } # [doc = " Convert to option, preferring the left leaf in case of a tie."] pub fn left_biased (self) -> Option < T > { match self { TokenAtOffset :: None => None , TokenAtOffset :: Single (node) => Some (node) , TokenAtOffset :: Between (left , _) => Some (left) , } } }
};
}
