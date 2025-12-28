macro_rules! deps {
    () => {
        View!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl View for [u8] { fn view_as < T > (& self , offset : usize) -> Option < & T > { unsafe { Some (& * self . is_proper_length_and_alignment (offset , 1) ?) } } fn view_as_slice_of < T > (& self , offset : usize , len : usize) -> Option < & [T] > { unsafe { Some (std :: slice :: from_raw_parts (self . is_proper_length_and_alignment (offset , len) ? , len ,)) } } fn copy_as < T > (& self , offset : usize) -> Option < T > { self . is_proper_length :: < T > (offset) ? ; unsafe { let mut data = std :: mem :: MaybeUninit :: zeroed () . assume_init () ; core :: ptr :: copy_nonoverlapping (self [offset ..] . as_ptr () , & mut data as * mut T as * mut u8 , size_of :: < T > () ,) ; Some (data) } } fn view_as_str (& self , offset : usize) -> Option < & [u8] > { let buffer = & self [offset ..] ; let pos = buffer . iter () . position (| c | * c == b'\0') ? ; Some (& self [offset .. offset + pos]) } fn is_proper_length < T > (& self , offset : usize) -> Option < () > { if offset + size_of :: < T > () <= self . len () { Some (()) } else { None } } fn is_proper_length_and_alignment < T > (& self , offset : usize , count : usize) -> Option < * const T > { self . is_proper_length :: < T > (offset * count) ? ; let ptr = & self [offset] as * const u8 as * const T ; if ptr . align_offset (align_of :: < T > ()) == 0 { Some (ptr) } else { None } } }
    };
}

impl_45!();