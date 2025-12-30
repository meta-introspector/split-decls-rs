// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl State { fn new_opendir (& self , dirp : * mut DIR) { self . dirs . write () . expect ("lock poisoned") . insert (dirp as usize , ReaddirState { iter : None , iter64 : None , } ,) ; } fn wrapped_readdir_inner < Dirent , GetIter , Readdir > (& self , dirp : * mut DIR , get_iter : GetIter , readdir : Readdir ,) -> * mut Dirent where Dirent : Copy , GetIter : FnOnce (& mut ReaddirState) -> & mut Option < DirentIterator < Dirent > > , Readdir : Fn () -> * mut Dirent , { self . dirs . write () . expect ("lock poisoned") . get_mut (& (dirp as usize)) . and_then (| dirstate | { let iter = get_iter (dirstate) ; if iter . is_none () { let mut entries = Vec :: new () ; loop { let entry = readdir () ; if entry . is_null () { break ; } entries . push (unsafe { * entry }) ; } entries . shuffle (& mut thread_rng ()) ; * iter = Some (DirentIterator { entries , index : 0 }) ; } let iter = iter . as_mut () . unwrap () ; info ! ("{:p}: reading entry {}/{}" , dirp , iter . index , iter . entries . len ()) ; iter . next () }) . unwrap_or (std :: ptr :: null_mut ()) } fn wrapped_readdir (& self , dirp : * mut DIR) -> * mut dirent { self . wrapped_readdir_inner (dirp , | dirstate | & mut dirstate . iter , | | unsafe { (self . readdir) (dirp) } ,) } fn wrapped_readdir64 (& self , dirp : * mut DIR) -> * mut dirent64 { self . wrapped_readdir_inner (dirp , | dirstate | & mut dirstate . iter64 , | | unsafe { (self . readdir64) (dirp) } ,) } }
};
}
