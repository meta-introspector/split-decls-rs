// Generated macro for impl_482 (impl)
macro_rules! Depcrate_dominator_treeimpl_482 {
() => {
// Module: crate::dominator_tree
// Provides: {"impl_482"}
// Dependencies: {}
# [doc = " Creating and computing the dominator tree pre-order."] impl DominatorTreePreorder { # [doc = " Create a new blank `DominatorTreePreorder`."] pub fn new () -> Self { Self { nodes : SecondaryMap :: new () , stack : Vec :: new () , } } # [doc = " Recompute this data structure to match `domtree`."] pub fn compute (& mut self , domtree : & DominatorTree) { self . nodes . clear () ; for & block in domtree . cfg_postorder () { if let Some (idom) = domtree . idom (block) { let sib = mem :: replace (& mut self . nodes [idom] . child , block . into ()) ; self . nodes [block] . sibling = sib ; } else { self . stack . push (block) ; } } debug_assert ! (self . stack . len () <= 1) ; let mut n = 0 ; while let Some (block) = self . stack . pop () { n += 1 ; let node = & mut self . nodes [block] ; node . pre_number = n ; node . pre_max = n ; if let Some (n) = node . sibling . expand () { self . stack . push (n) ; } if let Some (n) = node . child . expand () { self . stack . push (n) ; } } for & block in domtree . cfg_postorder () { if let Some (idom) = domtree . idom (block) { let pre_max = cmp :: max (self . nodes [block] . pre_max , self . nodes [idom] . pre_max) ; self . nodes [idom] . pre_max = pre_max ; } } } }
};
}
