// Generated macro for update_text_region_fn (function)
macro_rules! Depcrate_utilsupdate_text_region_fn {
() => {
// Module: crate::utils
// Provides: {"update_text_region_fn"}
// Dependencies: {}
pub fn update_text_region_fn (start : & str , end : & str , mut insert : impl FnMut (& mut String) ,) -> impl FnMut (& Path , & str , & mut String) -> UpdateStatus { move | path , src , dst | update_text_region (path , start , end , src , dst , & mut insert) }
};
}
