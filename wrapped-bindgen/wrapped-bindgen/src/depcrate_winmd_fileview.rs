// Generated macro for View (trait)
macro_rules! Depcrate_winmd_fileView {
() => {
// Module: crate::winmd::file
// Provides: {"View"}
// Dependencies: {}
trait View { fn view_as < T > (& self , offset : usize) -> Option < & T > ; fn view_as_slice_of < T > (& self , offset : usize , len : usize) -> Option < & [T] > ; fn copy_as < T : Copy > (& self , offset : usize) -> Option < T > ; fn view_as_str (& self , offset : usize) -> Option < & [u8] > ; fn is_proper_length < T > (& self , offset : usize) -> Option < () > ; fn is_proper_length_and_alignment < T > (& self , offset : usize , count : usize) -> Option < * const T > ; }
};
}
