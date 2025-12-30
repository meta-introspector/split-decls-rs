// Generated macro for impl_218 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_218 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_218"}
// Dependencies: {}
impl DisplayFmt { # [doc = " Print the permission with the format"] # [doc = " ` Res`/` Re*`/` Act`/` Frz`/` Dis` for accessed locations"] # [doc = " and `?Res`/`?Re*`/`?Act`/`?Frz`/`?Dis` for unaccessed locations."] fn print_perm (& self , perm : Option < LocationState >) -> String { if let Some (perm) = perm { format ! ("{ac}{st}" , ac = if perm . is_accessed () { self . accessed . yes } else { self . accessed . no } , st = perm . permission () . short_name () ,) } else { format ! ("{}{}" , self . accessed . meh , self . perm . uninit) } } # [doc = " Print the tag with the format `<XYZ>` if the tag is unnamed,"] # [doc = " and `<XYZ=name>` if the tag is named."] fn print_tag (& self , tag : BorTag , name : & Option < String >) -> String { let printable_tag = tag . get () ; if let Some (name) = name { format ! ("<{printable_tag}={name}>") } else { format ! ("<{printable_tag}>") } } # [doc = " Print extra text if the tag has a protector."] fn print_protector (& self , protector : Option < & ProtectorKind >) -> & 'static str { protector . map (| p | { match * p { ProtectorKind :: WeakProtector => " Weakly protected" , ProtectorKind :: StrongProtector => " Strongly protected" , } }) . unwrap_or ("") } }
};
}
