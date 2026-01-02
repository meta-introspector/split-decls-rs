mkuse!{use crate :: iter :: adapters :: zip :: try_get_unchecked ;}
mkuse!{use crate :: iter :: adapters :: { SourceIter , TrustedRandomAccess , TrustedRandomAccessNoCoerce } ;}
mkuse!{use crate :: iter :: { FusedIterator , InPlaceIterable , TrustedLen } ;}
mkuse!{use crate :: mem :: { MaybeUninit , SizedTypeProperties } ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ops :: Try ;}
mkuse!{use crate :: { array , ptr } ;}
mkitem!{mkstruct!{# [doc = " An iterator that copies the elements of an underlying iterator."] # [doc = ""] # [doc = " This `struct` is created by the [`copied`] method on [`Iterator`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`copied`]: Iterator::copied"] # [doc = " [`Iterator`]: trait.Iterator.html"] # [stable (feature = "iter_copied" , since = "1.36.0")] # [must_use = "iterators are lazy and do nothing unless consumed"] # [derive (Clone , Debug)] pub struct Copied < I > { it : I , }}}
mkitem!{mkimpl!{impl < I > Copied < I > { pub (in crate :: iter) fn new (it : I) -> Copied < I > { Copied { it } } }}}

macro_rules! copy_fold_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy_fold in module {}", module_path!());
    };
}

mkfn!{
    copy_fold_introspect!();
    fn copy_fold < T : Copy , Acc > (mut f : impl FnMut (Acc , T) -> Acc) -> impl FnMut (Acc , & T) -> Acc { move | acc , & elt | f (acc , elt) }
}

macro_rules! copy_try_fold_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy_try_fold in module {}", module_path!());
    };
}

mkfn!{
    copy_try_fold_introspect!();
    fn copy_try_fold < T : Copy , Acc , R > (mut f : impl FnMut (Acc , T) -> R) -> impl FnMut (Acc , & T) -> R { move | acc , & elt | f (acc , elt) }
}
mkitem!{mkimpl!{# [stable (feature = "iter_copied" , since = "1.36.0")] impl < 'a , I , T : 'a > Iterator for Copied < I > where I : Iterator < Item = & 'a T > , T : Copy , { type Item = T ; fn next (& mut self) -> Option < T > { self . it . next () . copied () } fn next_chunk < const N : usize > (& mut self ,) -> Result < [Self :: Item ; N] , array :: IntoIter < Self :: Item , N > > where Self : Sized , { < I as SpecNextChunk < '_ , N , T > > :: spec_next_chunk (& mut self . it) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } fn try_fold < B , F , R > (& mut self , init : B , f : F) -> R where Self : Sized , F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { self . it . try_fold (init , copy_try_fold (f)) } fn fold < Acc , F > (self , init : Acc , f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { self . it . fold (init , copy_fold (f)) } fn nth (& mut self , n : usize) -> Option < T > { self . it . nth (n) . copied () } fn last (self) -> Option < T > { self . it . last () . copied () } fn count (self) -> usize { self . it . count () } # [inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . it . advance_by (n) } unsafe fn __iterator_get_unchecked (& mut self , idx : usize) -> T where Self : TrustedRandomAccessNoCoerce , { * unsafe { try_get_unchecked (& mut self . it , idx) } } }}}
mkitem!{mkimpl!{# [stable (feature = "iter_copied" , since = "1.36.0")] impl < 'a , I , T : 'a > DoubleEndedIterator for Copied < I > where I : DoubleEndedIterator < Item = & 'a T > , T : Copy , { fn next_back (& mut self) -> Option < T > { self . it . next_back () . copied () } fn try_rfold < B , F , R > (& mut self , init : B , f : F) -> R where Self : Sized , F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { self . it . try_rfold (init , copy_try_fold (f)) } fn rfold < Acc , F > (self , init : Acc , f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { self . it . rfold (init , copy_fold (f)) } # [inline] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . it . advance_back_by (n) } }}}
mkitem!{mkimpl!{# [stable (feature = "iter_copied" , since = "1.36.0")] impl < 'a , I , T : 'a > ExactSizeIterator for Copied < I > where I : ExactSizeIterator < Item = & 'a T > , T : Copy , { fn len (& self) -> usize { self . it . len () } fn is_empty (& self) -> bool { self . it . is_empty () } }}}
mkitem!{mkimpl!{# [stable (feature = "iter_copied" , since = "1.36.0")] impl < 'a , I , T : 'a > FusedIterator for Copied < I > where I : FusedIterator < Item = & 'a T > , T : Copy , { }}}
mkitem!{mkimpl!{# [doc (hidden)] # [unstable (feature = "trusted_random_access" , issue = "none")] unsafe impl < I > TrustedRandomAccess for Copied < I > where I : TrustedRandomAccess { }}}
mkitem!{mkimpl!{# [doc (hidden)] # [unstable (feature = "trusted_random_access" , issue = "none")] unsafe impl < I > TrustedRandomAccessNoCoerce for Copied < I > where I : TrustedRandomAccessNoCoerce , { const MAY_HAVE_SIDE_EFFECT : bool = I :: MAY_HAVE_SIDE_EFFECT ; }}}
mkitem!{mkimpl!{# [stable (feature = "iter_copied" , since = "1.36.0")] unsafe impl < 'a , I , T : 'a > TrustedLen for Copied < I > where I : TrustedLen < Item = & 'a T > , T : Copy , { }}}
mkitem!{mktrait!{trait SpecNextChunk < 'a , const N : usize , T : 'a > : Iterator < Item = & 'a T > where T : Copy , { fn spec_next_chunk (& mut self) -> Result < [T ; N] , array :: IntoIter < T , N > > ; }}}
mkitem!{mkimpl!{impl < 'a , const N : usize , I , T : 'a > SpecNextChunk < 'a , N , T > for I where I : Iterator < Item = & 'a T > , T : Copy , { default fn spec_next_chunk (& mut self) -> Result < [T ; N] , array :: IntoIter < T , N > > { array :: iter_next_chunk (& mut self . copied ()) } }}}
mkitem!{mkimpl!{impl < 'a , const N : usize , T : 'a > SpecNextChunk < 'a , N , T > for crate :: slice :: Iter < 'a , T > where T : Copy , { fn spec_next_chunk (& mut self) -> Result < [T ; N] , array :: IntoIter < T , N > > { let mut raw_array = [const { MaybeUninit :: uninit () } ; N] ; let len = self . len () ; if T :: IS_ZST { if len < N { let _ = self . advance_by (len) ; return Err (unsafe { array :: IntoIter :: new_unchecked (raw_array , 0 .. len) }) ; } let _ = self . advance_by (N) ; return Ok (unsafe { MaybeUninit :: array_assume_init (raw_array) }) ; } if len < N { unsafe { ptr :: copy_nonoverlapping (self . as_ref () . as_ptr () , raw_array . as_mut_ptr () as * mut T , len ,) ; let _ = self . advance_by (len) ; return Err (array :: IntoIter :: new_unchecked (raw_array , 0 .. len)) ; } } unsafe { ptr :: copy_nonoverlapping (self . as_ref () . as_ptr () , raw_array . as_mut_ptr () as * mut T , N) ; let _ = self . advance_by (N) ; Ok (MaybeUninit :: array_assume_init (raw_array)) } } }}}
mkitem!{mkimpl!{# [stable (feature = "default_iters" , since = "1.70.0")] impl < I : Default > Default for Copied < I > { # [doc = " Creates a `Copied` iterator from the default value of `I`"] # [doc = " ```"] # [doc = " # use core::slice;"] # [doc = " # use core::iter::Copied;"] # [doc = " let iter: Copied<slice::Iter<'_, u8>> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { Self :: new (Default :: default ()) } }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I > SourceIter for Copied < I > where I : SourceIter , { type Source = I :: Source ; # [inline] unsafe fn as_inner (& mut self) -> & mut I :: Source { unsafe { SourceIter :: as_inner (& mut self . it) } } }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I : InPlaceIterable > InPlaceIterable for Copied < I > { const EXPAND_BY : Option < NonZero < usize > > = I :: EXPAND_BY ; const MERGE_BY : Option < NonZero < usize > > = I :: MERGE_BY ; }}}