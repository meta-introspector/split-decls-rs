// Generated macro for dotnet_aligned_array (function)
macro_rules! Depcrate_world_generatordotnet_aligned_array {
() => {
// Module: crate::world_generator
// Provides: {"dotnet_aligned_array"}
// Dependencies: {}
pub fn dotnet_aligned_array (array_size : usize , required_alignment : usize) -> (usize , String) { match required_alignment { 1 => (array_size , "byte" . to_owned ()) , 2 => ((array_size + 1) / 2 , "ushort" . to_owned ()) , 4 => ((array_size + 3) / 4 , "uint" . to_owned ()) , 8 => ((array_size + 7) / 8 , "ulong" . to_owned ()) , _ => todo ! ("unsupported return_area_align {}" , required_alignment) , } }
};
}
