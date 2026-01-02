mkuse!{use crate :: fmt ;}
mkuse!{use crate :: iter :: adapters :: zip :: try_get_unchecked ;}
mkuse!{use crate :: iter :: adapters :: { SourceIter , TrustedRandomAccess , TrustedRandomAccessNoCoerce } ;}
mkuse!{use crate :: iter :: { FusedIterator , InPlaceIterable , TrustedFused , TrustedLen , UncheckedIterator } ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ops :: Try ;}
mkitem!{mkstruct!{# [doc = " An iterator that maps the values of `iter` with `f`."] # [doc = ""] # [doc = " This `struct` is created by the [`map`] method on [`Iterator`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`map`]: Iterator::map"] # [doc = " [`Iterator`]: trait.Iterator.html"] # [doc = ""] # [doc = " # Notes about side effects"] # [doc = ""] # [doc = " The [`map`] iterator implements [`DoubleEndedIterator`], meaning that"] # [doc = " you can also [`map`] backwards:"] # [doc = ""] # [doc = " ```rust"] # [doc = " let v: Vec<i32> = [1, 2, 3].into_iter().map(|x| x + 1).rev().collect();"] # [doc = ""] # [doc = " assert_eq!(v, [4, 3, 2]);"] # [doc = " ```"] # [doc = ""] # [doc = " [`DoubleEndedIterator`]: trait.DoubleEndedIterator.html"] # [doc = ""] # [doc = " But if your closure has state, iterating backwards may act in a way you do"] # [doc = " not expect. Let's go through an example. First, in the forward direction:"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut c = 0;"] # [doc = ""] # [doc = " for pair in ['a', 'b', 'c'].into_iter()"] # [doc = "                                .map(|letter| { c += 1; (letter, c) }) {"] # [doc = "     println!(\"{pair:?}\");"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This will print `('a', 1), ('b', 2), ('c', 3)`."] # [doc = ""] # [doc = " Now consider this twist where we add a call to `rev`. This version will"] # [doc = " print `('c', 1), ('b', 2), ('a', 3)`. Note that the letters are reversed,"] # [doc = " but the values of the counter still go in order. This is because `map()` is"] # [doc = " still being called lazily on each item, but we are popping items off the"] # [doc = " back of the vector now, instead of shifting them from the front."] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut c = 0;"] # [doc = ""] # [doc = " for pair in ['a', 'b', 'c'].into_iter()"] # [doc = "                                .map(|letter| { c += 1; (letter, c) })"] # [doc = "                                .rev() {"] # [doc = "     println!(\"{pair:?}\");"] # [doc = " }"] # [doc = " ```"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Clone)] pub struct Map < I , F > { pub (crate) iter : I , f : F , }}}
mkitem!{mkimpl!{impl < I , F > Map < I , F > { pub (in crate :: iter) fn new (iter : I , f : F) -> Map < I , F > { Map { iter , f } } pub (crate) fn into_inner (self) -> I { self . iter } }}}
mkitem!{mkimpl!{# [stable (feature = "core_impl_debug" , since = "1.9.0")] impl < I : fmt :: Debug , F > fmt :: Debug for Map < I , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Map") . field ("iter" , & self . iter) . finish () } }}}

macro_rules! map_fold_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function map_fold in module {}", module_path!());
    };
}

mkfn!{
    map_fold_introspect!();
    fn map_fold < T , B , Acc > (mut f : impl FnMut (T) -> B , mut g : impl FnMut (Acc , B) -> Acc ,) -> impl FnMut (Acc , T) -> Acc { move | acc , elt | g (acc , f (elt)) }
}

macro_rules! map_try_fold_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function map_try_fold in module {}", module_path!());
    };
}

mkfn!{
    map_try_fold_introspect!();
    fn map_try_fold < 'a , T , B , Acc , R > (f : & 'a mut impl FnMut (T) -> B , mut g : impl FnMut (Acc , B) -> R + 'a ,) -> impl FnMut (Acc , T) -> R + 'a { move | acc , elt | g (acc , f (elt)) }
}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < B , I : Iterator , F > Iterator for Map < I , F > where F : FnMut (I :: Item) -> B , { type Item = B ; # [inline] fn next (& mut self) -> Option < B > { self . iter . next () . map (& mut self . f) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } fn try_fold < Acc , G , R > (& mut self , init : Acc , g : G) -> R where Self : Sized , G : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { self . iter . try_fold (init , map_try_fold (& mut self . f , g)) } fn fold < Acc , G > (self , init : Acc , g : G) -> Acc where G : FnMut (Acc , Self :: Item) -> Acc , { self . iter . fold (init , map_fold (self . f , g)) } # [inline] unsafe fn __iterator_get_unchecked (& mut self , idx : usize) -> B where Self : TrustedRandomAccessNoCoerce , { unsafe { (self . f) (try_get_unchecked (& mut self . iter , idx)) } } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < B , I : DoubleEndedIterator , F > DoubleEndedIterator for Map < I , F > where F : FnMut (I :: Item) -> B , { # [inline] fn next_back (& mut self) -> Option < B > { self . iter . next_back () . map (& mut self . f) } fn try_rfold < Acc , G , R > (& mut self , init : Acc , g : G) -> R where Self : Sized , G : FnMut (Acc , Self :: Item) -> R , R : Try < Output = Acc > , { self . iter . try_rfold (init , map_try_fold (& mut self . f , g)) } fn rfold < Acc , G > (self , init : Acc , g : G) -> Acc where G : FnMut (Acc , Self :: Item) -> Acc , { self . iter . rfold (init , map_fold (self . f , g)) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < B , I : ExactSizeIterator , F > ExactSizeIterator for Map < I , F > where F : FnMut (I :: Item) -> B , { fn len (& self) -> usize { self . iter . len () } fn is_empty (& self) -> bool { self . iter . is_empty () } }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl < B , I : FusedIterator , F > FusedIterator for Map < I , F > where F : FnMut (I :: Item) -> B { }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "trusted_fused")] unsafe impl < I : TrustedFused , F > TrustedFused for Map < I , F > { }}}
mkitem!{mkimpl!{# [unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < B , I , F > TrustedLen for Map < I , F > where I : TrustedLen , F : FnMut (I :: Item) -> B , { }}}
mkitem!{mkimpl!{impl < B , I , F > UncheckedIterator for Map < I , F > where I : UncheckedIterator , F : FnMut (I :: Item) -> B , { unsafe fn next_unchecked (& mut self) -> B { let item = unsafe { self . iter . next_unchecked () } ; (self . f) (item) } }}}
mkitem!{mkimpl!{# [doc (hidden)] # [unstable (feature = "trusted_random_access" , issue = "none")] unsafe impl < I , F > TrustedRandomAccess for Map < I , F > where I : TrustedRandomAccess { }}}
mkitem!{mkimpl!{# [doc (hidden)] # [unstable (feature = "trusted_random_access" , issue = "none")] unsafe impl < I , F > TrustedRandomAccessNoCoerce for Map < I , F > where I : TrustedRandomAccessNoCoerce , { const MAY_HAVE_SIDE_EFFECT : bool = true ; }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I , F > SourceIter for Map < I , F > where I : SourceIter , { type Source = I :: Source ; # [inline] unsafe fn as_inner (& mut self) -> & mut I :: Source { unsafe { SourceIter :: as_inner (& mut self . iter) } } }}}
mkitem!{mkimpl!{# [unstable (issue = "none" , feature = "inplace_iteration")] unsafe impl < I : InPlaceIterable , F > InPlaceIterable for Map < I , F > { const EXPAND_BY : Option < NonZero < usize > > = I :: EXPAND_BY ; const MERGE_BY : Option < NonZero < usize > > = I :: MERGE_BY ; }}}