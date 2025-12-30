// Generated macro for impl_78 (impl)
macro_rules! Depcrate_core_read_dir_iterimpl_78 {
() => {
// Module: crate::core::read_dir_iter
// Provides: {"impl_78"}
// Dependencies: {}
impl < C : ClientState > Iterator for ReadDirIter < C > { type Item = Result < ReadDir < C > > ; fn next (& mut self) -> Option < Self :: Item > { match self { ReadDirIter :: Walk { read_dir_spec_stack , core_read_dir_callback , } => { let read_dir_spec = read_dir_spec_stack . pop () ? ; let read_dir_result = core_read_dir_callback (read_dir_spec) ; if let Ok (read_dir) = read_dir_result . as_ref () { for each_spec in read_dir . read_children_specs () . collect :: < Vec < _ > > () . into_iter () . rev () { read_dir_spec_stack . push (each_spec) ; } } Some (read_dir_result) } ReadDirIter :: ParWalk { read_dir_result_iter , } => read_dir_result_iter . next () . map (| read_dir_result | read_dir_result . value) , } } }
};
}
