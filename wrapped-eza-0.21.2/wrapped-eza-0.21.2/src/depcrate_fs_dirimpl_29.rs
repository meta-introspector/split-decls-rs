// Generated macro for impl_29 (impl)
macro_rules! Depcrate_fs_dirimpl_29 {
() => {
// Module: crate::fs::dir
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'dir , 'ig > Files < 'dir , 'ig > { fn parent (& self) -> PathBuf { self . dir . path . join ("..") } # [doc = " Go through the directory until we encounter a file we can list (which"] # [doc = " varies depending on the dotfile visibility flag)"] fn next_visible_file (& mut self) -> Option < File < 'dir > > { loop { if let Some (entry) = self . inner . next () { let path = entry . path () ; let filename = File :: filename (& path) ; if ! self . dotfiles && filename . starts_with ('.') { continue ; } # [cfg (windows)] if ! self . dotfiles && filename . starts_with ('_') { continue ; } if self . git_ignoring { let git_status = self . git . map (| g | g . get (& path , false)) . unwrap_or_default () ; if git_status . unstaged == GitStatus :: Ignored { continue ; } } let file = File :: from_args (path , self . dir , filename , self . deref_links , self . total_size , entry . file_type () . ok () ,) ; # [cfg (windows)] if ! self . dotfiles && file . attributes () . map_or (false , | a | a . hidden) { continue ; } return Some (file) ; } return None ; } } }
};
}
