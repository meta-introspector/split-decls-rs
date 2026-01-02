mkuse!{use core :: alloc :: { Allocator , Layout } ;}
mkuse!{use core :: iter :: { InPlaceIterable , SourceIter , TrustedRandomAccessNoCoerce } ;}
mkuse!{use core :: marker :: PhantomData ;}
mkuse!{use core :: mem :: { self , ManuallyDrop , SizedTypeProperties } ;}
mkuse!{use core :: num :: NonZero ;}
mkuse!{use core :: ptr ;}
mkuse!{use super :: { InPlaceDrop , InPlaceDstDataSrcBufDrop , SpecFromIter , SpecFromIterNested , Vec } ;}
mkuse!{use crate :: alloc :: { Global , handle_alloc_error } ;}

macro_rules! in_place_collectible_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function in_place_collectible in module {}", module_path!());
    };
}

mkfn!{
    in_place_collectible_introspect!();
    const fn in_place_collectible < DEST , SRC > (step_merge : Option < NonZero < usize > > , step_expand : Option < NonZero < usize > > ,) -> bool { if const { SRC :: IS_ZST || DEST :: IS_ZST || align_of :: < SRC > () != align_of :: < DEST > () } { return false ; } match (step_merge , step_expand) { (Some (step_merge) , Some (step_expand)) => { size_of :: < SRC > () * step_merge . get () >= size_of :: < DEST > () * step_expand . get () } _ => false , } }
}

macro_rules! needs_realloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function needs_realloc in module {}", module_path!());
    };
}

mkfn!{
    needs_realloc_introspect!();
    const fn needs_realloc < SRC , DEST > (src_cap : usize , dst_cap : usize) -> bool { if const { align_of :: < SRC > () != align_of :: < DEST > () } { panic ! ("in_place_collectible() prevents this") ; } if const { let src_sz = size_of :: < SRC > () ; let dest_sz = size_of :: < DEST > () ; dest_sz != 0 && src_sz % dest_sz == 0 } { return false ; } src_cap > 0 && src_cap * size_of :: < SRC > () != dst_cap * size_of :: < DEST > () }
}
mkitem!{mktrait!{# [doc = " This provides a shorthand for the source type since local type aliases aren't a thing."] # [rustc_specialization_trait] trait InPlaceCollect : SourceIter < Source : AsVecIntoIter > + InPlaceIterable { type Src ; }}}
mkitem!{mkimpl!{impl < T > InPlaceCollect for T where T : SourceIter < Source : AsVecIntoIter > + InPlaceIterable , { type Src = < < T as SourceIter > :: Source as AsVecIntoIter > :: Item ; }}}
mkitem!{mkimpl!{impl < T , I > SpecFromIter < T , I > for Vec < T > where I : Iterator < Item = T > + InPlaceCollect , < I as SourceIter > :: Source : AsVecIntoIter , { # [track_caller] default fn from_iter (iterator : I) -> Self { let fun : fn (I) -> Vec < T > = const { if in_place_collectible :: < T , I :: Src > (I :: MERGE_BY , I :: EXPAND_BY) { from_iter_in_place } else { SpecFromIterNested :: < T , I > :: from_iter } } ; fun (iterator) } }}}

macro_rules! from_iter_in_place_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_iter_in_place in module {}", module_path!());
    };
}

mkfn!{
    from_iter_in_place_introspect!();
    # [track_caller] fn from_iter_in_place < I , T > (mut iterator : I) -> Vec < T > where I : Iterator < Item = T > + InPlaceCollect , < I as SourceIter > :: Source : AsVecIntoIter , { let (src_buf , src_ptr , src_cap , mut dst_buf , dst_end , dst_cap) = unsafe { let inner = iterator . as_inner () . as_into_iter () ; (inner . buf , inner . ptr , inner . cap , inner . buf . cast :: < T > () , inner . end as * const T , inner . cap . unchecked_mul (size_of :: < I :: Src > ()) / size_of :: < T > () ,) } ; let len = unsafe { SpecInPlaceCollect :: collect_in_place (& mut iterator , dst_buf . as_ptr () as * mut T , dst_end) } ; let src = unsafe { iterator . as_inner () . as_into_iter () } ; debug_assert_eq ! (src_buf , src . buf) ; if src . ptr != src_ptr { debug_assert ! (unsafe { dst_buf . add (len) . cast () } <= src . ptr , "InPlaceIterable contract violation, write pointer advanced beyond read pointer") ; } let dst_guard = InPlaceDstDataSrcBufDrop { ptr : dst_buf , len , src_cap , src : PhantomData :: < I :: Src > } ; src . forget_allocation_drop_remaining () ; if needs_realloc :: < I :: Src , T > (src_cap , dst_cap) { let alloc = Global ; debug_assert_ne ! (src_cap , 0) ; debug_assert_ne ! (dst_cap , 0) ; unsafe { let src_align = align_of :: < I :: Src > () ; let src_size = size_of :: < I :: Src > () . unchecked_mul (src_cap) ; let old_layout = Layout :: from_size_align_unchecked (src_size , src_align) ; let dst_align = align_of :: < T > () ; let dst_size = size_of :: < T > () . unchecked_mul (dst_cap) ; let new_layout = Layout :: from_size_align_unchecked (dst_size , dst_align) ; let result = alloc . shrink (dst_buf . cast () , old_layout , new_layout) ; let Ok (reallocated) = result else { handle_alloc_error (new_layout) } ; dst_buf = reallocated . cast :: < T > () ; } } else { debug_assert_eq ! (src_cap * size_of ::< I :: Src > () , dst_cap * size_of ::< T > ()) ; } mem :: forget (dst_guard) ; let vec = unsafe { Vec :: from_parts (dst_buf , len , dst_cap) } ; vec }
}

macro_rules! write_in_place_with_drop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_in_place_with_drop in module {}", module_path!());
    };
}

mkfn!{
    write_in_place_with_drop_introspect!();
    fn write_in_place_with_drop < T > (src_end : * const T ,) -> impl FnMut (InPlaceDrop < T > , T) -> Result < InPlaceDrop < T > , ! > { move | mut sink , item | { unsafe { debug_assert ! (sink . dst as * const _ <= src_end , "InPlaceIterable contract violation") ; ptr :: write (sink . dst , item) ; sink . dst = sink . dst . add (1) ; } Ok (sink) } }
}
mkitem!{mktrait!{# [doc = " Helper trait to hold specialized implementations of the in-place iterate-collect loop"] trait SpecInPlaceCollect < T , I > : Iterator < Item = T > { # [doc = " Collects an iterator (`self`) into the destination buffer (`dst`) and returns the number of items"] # [doc = " collected. `end` is the last writable element of the allocation and used for bounds checks."] # [doc = ""] # [doc = " This method is specialized and one of its implementations makes use of"] # [doc = " `Iterator::__iterator_get_unchecked` calls with a `TrustedRandomAccessNoCoerce` bound"] # [doc = " on `I` which means the caller of this method must take the safety conditions"] # [doc = " of that trait into consideration."] unsafe fn collect_in_place (& mut self , dst : * mut T , end : * const T) -> usize ; }}}
mkitem!{mkimpl!{impl < T , I > SpecInPlaceCollect < T , I > for I where I : Iterator < Item = T > , { # [inline] default unsafe fn collect_in_place (& mut self , dst_buf : * mut T , end : * const T) -> usize { let sink = InPlaceDrop { inner : dst_buf , dst : dst_buf } ; let sink = self . try_fold :: < _ , _ , Result < _ , ! > > (sink , write_in_place_with_drop (end)) . into_ok () ; unsafe { ManuallyDrop :: new (sink) . dst . offset_from_unsigned (dst_buf) } } }}}
mkitem!{mkimpl!{impl < T , I > SpecInPlaceCollect < T , I > for I where I : Iterator < Item = T > + TrustedRandomAccessNoCoerce , { # [inline] unsafe fn collect_in_place (& mut self , dst_buf : * mut T , end : * const T) -> usize { let len = self . size () ; let mut drop_guard = InPlaceDrop { inner : dst_buf , dst : dst_buf } ; for i in 0 .. len { unsafe { let dst = dst_buf . add (i) ; debug_assert ! (dst as * const _ <= end , "InPlaceIterable contract violation") ; ptr :: write (dst , self . __iterator_get_unchecked (i)) ; drop_guard . dst = dst . add (1) ; } } mem :: forget (drop_guard) ; len } }}}
mkitem!{mktrait!{# [doc = " Internal helper trait for in-place iteration specialization."] # [doc = ""] # [doc = " Currently this is only implemented by [`vec::IntoIter`] - returning a reference to itself - and"] # [doc = " [`binary_heap::IntoIter`] which returns a reference to its inner representation."] # [doc = ""] # [doc = " Since this is an internal trait it hides the implementation detail `binary_heap::IntoIter`"] # [doc = " uses `vec::IntoIter` internally."] # [doc = ""] # [doc = " [`vec::IntoIter`]: super::IntoIter"] # [doc = " [`binary_heap::IntoIter`]: crate::collections::binary_heap::IntoIter"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " In-place iteration relies on implementation details of `vec::IntoIter`, most importantly that"] # [doc = " it does not create references to the whole allocation during iteration, only raw pointers"] # [rustc_specialization_trait] pub (crate) unsafe trait AsVecIntoIter { type Item ; fn as_into_iter (& mut self) -> & mut super :: IntoIter < Self :: Item > ; }}}