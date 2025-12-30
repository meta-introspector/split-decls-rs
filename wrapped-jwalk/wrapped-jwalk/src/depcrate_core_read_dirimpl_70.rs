// Generated macro for impl_70 (impl)
macro_rules! Depcrate_core_read_dirimpl_70 {
() => {
// Module: crate::core::read_dir
// Provides: {"impl_70"}
// Dependencies: {}
impl < C : ClientState > ReadDir < C > { pub fn new (read_dir_state : C :: ReadDirState , results_list : Vec < Result < DirEntry < C > > > ,) -> ReadDir < C > { ReadDir { read_dir_state , results_list , } } pub fn read_children_specs (& self) -> impl Iterator < Item = ReadDirSpec < C > > + '_ { self . results_list . iter () . filter_map (move | each | { each . as_ref () . ok () ? . read_children_spec (self . read_dir_state . clone ()) }) } pub fn ordered_read_children_specs (& self , index_path : & IndexPath ,) -> Vec < Ordered < ReadDirSpec < C > > > { self . read_children_specs () . enumerate () . map (| (i , spec) | Ordered :: new (spec , index_path . adding (i) , 0)) . collect () } }
};
}
