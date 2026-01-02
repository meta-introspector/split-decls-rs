mkuse!{use core :: iter :: TrustedLen ;}
mkuse!{use core :: slice :: { self } ;}
mkuse!{use super :: { IntoIter , Vec } ;}
mkuse!{use crate :: alloc :: Allocator ;}
mkitem!{mktrait!{pub (super) trait SpecExtend < T , I > { # [track_caller] fn spec_extend (& mut self , iter : I) ; }}}
mkitem!{mkimpl!{impl < T , I , A : Allocator > SpecExtend < T , I > for Vec < T , A > where I : Iterator < Item = T > , { # [track_caller] default fn spec_extend (& mut self , iter : I) { self . extend_desugared (iter) } }}}
mkitem!{mkimpl!{impl < T , I , A : Allocator > SpecExtend < T , I > for Vec < T , A > where I : TrustedLen < Item = T > , { # [track_caller] default fn spec_extend (& mut self , iterator : I) { self . extend_trusted (iterator) } }}}
mkitem!{mkimpl!{impl < T , A : Allocator > SpecExtend < T , IntoIter < T > > for Vec < T , A > { # [track_caller] fn spec_extend (& mut self , mut iterator : IntoIter < T >) { unsafe { self . append_elements (iterator . as_slice () as _) ; } iterator . forget_remaining_elements () ; } }}}
mkitem!{mkimpl!{impl < 'a , T : 'a , I , A : Allocator > SpecExtend < & 'a T , I > for Vec < T , A > where I : Iterator < Item = & 'a T > , T : Clone , { # [track_caller] default fn spec_extend (& mut self , iterator : I) { self . spec_extend (iterator . cloned ()) } }}}
mkitem!{mkimpl!{impl < 'a , T : 'a , A : Allocator > SpecExtend < & 'a T , slice :: Iter < 'a , T > > for Vec < T , A > where T : Copy , { # [track_caller] fn spec_extend (& mut self , iterator : slice :: Iter < 'a , T >) { let slice = iterator . as_slice () ; unsafe { self . append_elements (slice) } ; } }}}