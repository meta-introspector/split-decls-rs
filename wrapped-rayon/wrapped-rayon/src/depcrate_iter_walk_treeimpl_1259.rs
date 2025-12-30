// Generated macro for impl_1259 (impl)
macro_rules! Depcrate_iter_walk_treeimpl_1259 {
() => {
// Module: crate::iter::walk_tree
// Provides: {"impl_1259"}
// Dependencies: {}
impl < S , B , I > UnindexedProducer for WalkTreePostfixProducer < '_ , S , B > where S : Send , B : Fn (& S) -> I + Send + Sync , I : IntoIterator < Item = S > , { type Item = S ; fn split (mut self) -> (Self , Option < Self >) { while self . to_explore . len () == 1 { let front_node = self . to_explore . pop () . unwrap () ; self . to_explore . extend ((self . children_of) (& front_node) . into_iter ()) ; self . seen . push (front_node) ; } let right_children = split_vec (& mut self . to_explore) ; let right = right_children . map (| c | { let right_seen = std :: mem :: take (& mut self . seen) ; WalkTreePostfixProducer { to_explore : c , seen : right_seen , children_of : self . children_of , } }) . or_else (| | { let right_seen = split_vec (& mut self . seen) ; right_seen . map (| mut s | { std :: mem :: swap (& mut self . seen , & mut s) ; WalkTreePostfixProducer { to_explore : Default :: default () , seen : s , children_of : self . children_of , } }) }) ; (self , right) } fn fold_with < F > (self , mut folder : F) -> F where F : Folder < Self :: Item > , { for e in self . to_explore { folder = consume_rec_postfix (& self . children_of , e , folder) ; if folder . full () { return folder ; } } folder . consume_iter (self . seen . into_iter () . rev ()) } }
};
}
