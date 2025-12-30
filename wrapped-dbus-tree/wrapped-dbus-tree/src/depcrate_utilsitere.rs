// Generated macro for IterE (enum)
macro_rules! Depcrate_utilsIterE {
() => {
// Module: crate::utils
// Provides: {"IterE"}
// Dependencies: {}
# [derive (Clone , Debug)] pub enum IterE < 'a , V : 'a > { Path (btree_map :: Values < 'a , Arc < Path < 'static > > , Arc < V > >) , Iface (btree_map :: Values < 'a , Arc < IfaceName < 'static > > , Arc < V > >) , Member (btree_map :: Values < 'a , Member < 'static > , Arc < V > >) , String (btree_map :: Values < 'a , String , Arc < V > >) , }
};
}
