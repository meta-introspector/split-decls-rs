mkuse!{use core :: marker :: PhantomData ;}
mkuse!{use core :: ptr :: { self , NonNull , drop_in_place } ;}
mkuse!{use core :: slice :: { self } ;}
mkuse!{use crate :: alloc :: Global ;}
mkuse!{use crate :: raw_vec :: RawVec ;}
mkitem!{mkstruct!{pub (super) struct InPlaceDrop < T > { pub (super) inner : * mut T , pub (super) dst : * mut T , }}}
mkitem!{mkimpl!{impl < T > InPlaceDrop < T > { fn len (& self) -> usize { unsafe { self . dst . offset_from_unsigned (self . inner) } } }}}
mkitem!{mkimpl!{impl < T > Drop for InPlaceDrop < T > { # [inline] fn drop (& mut self) { unsafe { ptr :: drop_in_place (slice :: from_raw_parts_mut (self . inner , self . len ())) ; } } }}}
mkitem!{mkstruct!{pub (super) struct InPlaceDstDataSrcBufDrop < Src , Dest > { pub (super) ptr : NonNull < Dest > , pub (super) len : usize , pub (super) src_cap : usize , pub (super) src : PhantomData < Src > , }}}
mkitem!{mkimpl!{impl < Src , Dest > Drop for InPlaceDstDataSrcBufDrop < Src , Dest > { # [inline] fn drop (& mut self) { unsafe { let _drop_allocation = RawVec :: < Src > :: from_nonnull_in (self . ptr . cast :: < Src > () , self . src_cap , Global) ; drop_in_place (core :: ptr :: slice_from_raw_parts_mut :: < Dest > (self . ptr . as_ptr () , self . len)) ; } ; } }}}