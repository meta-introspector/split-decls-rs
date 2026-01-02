mkuse!{use crate :: ffi :: OsString ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ops :: Try ;}
mkuse!{use crate :: { array , fmt , vec } ;}
mkitem!{mkstruct!{pub struct Args { iter : vec :: IntoIter < OsString > , }}}
mkitem!{mkimpl!{impl ! Send for Args { }}}
mkitem!{mkimpl!{impl ! Sync for Args { }}}
mkitem!{mkimpl!{impl Args { # [inline] pub fn new (args : Vec < OsString >) -> Self { Args { iter : args . into_iter () } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Args { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . iter . as_slice () . fmt (f) } }}}
mkitem!{mkimpl!{impl Iterator for Args { type Item = OsString ; # [inline] fn next (& mut self) -> Option < OsString > { self . iter . next () } # [inline] fn next_chunk < const N : usize > (& mut self ,) -> Result < [OsString ; N] , array :: IntoIter < OsString , N > > { self . iter . next_chunk () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [inline] fn count (self) -> usize { self . iter . len () } # [inline] fn last (self) -> Option < OsString > { self . iter . last () } # [inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . iter . advance_by (n) } # [inline] fn try_fold < B , F , R > (& mut self , init : B , f : F) -> R where F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { self . iter . try_fold (init , f) } # [inline] fn fold < B , F > (self , init : B , f : F) -> B where F : FnMut (B , Self :: Item) -> B , { self . iter . fold (init , f) } }}}
mkitem!{mkimpl!{impl DoubleEndedIterator for Args { # [inline] fn next_back (& mut self) -> Option < OsString > { self . iter . next_back () } # [inline] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . iter . advance_back_by (n) } }}}
mkitem!{mkimpl!{impl ExactSizeIterator for Args { # [inline] fn len (& self) -> usize { self . iter . len () } # [inline] fn is_empty (& self) -> bool { self . iter . is_empty () } }}}