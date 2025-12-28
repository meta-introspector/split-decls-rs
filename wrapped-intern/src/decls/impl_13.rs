macro_rules! deps {
    () => {
        TaggedArcPtr!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl TaggedArcPtr { const BOOL_BITS : usize = true as usize ; const fn non_arc (r : & 'static & 'static str) -> Self { assert ! (align_of ::<&'static &'static str > () . trailing_zeros () as usize > Self :: BOOL_BITS) ; let packed = unsafe { NonNull :: new_unchecked ((r as * const & str) . cast :: < * const str > () . cast_mut ()) } ; Self { packed } } fn arc (arc : Arc < Box < str > >) -> Self { assert ! (align_of ::<&'static &'static str > () . trailing_zeros () as usize > Self :: BOOL_BITS) ; Self { packed : Self :: pack_arc (unsafe { NonNull :: new_unchecked (Arc :: into_raw (arc) . cast_mut () . cast ()) } ,) , } } # [doc = " Retrieves the tag."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " You can only drop the `Arc` if the instance is dropped."] # [inline] pub (crate) unsafe fn try_as_arc_owned (self) -> Option < ManuallyDrop < Arc < Box < str > > > > { let tag = self . packed . as_ptr () . addr () & Self :: BOOL_BITS ; if tag != 0 { Some (ManuallyDrop :: new (unsafe { Arc :: from_raw (self . pointer () . as_ptr () . cast :: < Box < str > > ()) })) } else { None } } # [inline] fn pack_arc (ptr : NonNull < * const str >) -> NonNull < * const str > { let packed_tag = true as usize ; unsafe { NonNull :: new_unchecked (ptr . as_ptr () . map_addr (| addr | addr | packed_tag)) } } # [inline] pub (crate) fn pointer (self) -> NonNull < * const str > { unsafe { NonNull :: new_unchecked (self . packed . as_ptr () . map_addr (| addr | addr & ! Self :: BOOL_BITS)) } } # [inline] pub (crate) fn as_str (& self) -> & str { unsafe { * self . pointer () . as_ptr () . cast :: < & str > () } } }
    };
}

impl_13!()