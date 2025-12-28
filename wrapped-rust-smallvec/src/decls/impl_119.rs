macro_rules! deps {
    () => {
        SmallVec!();
        DropDealloc!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        # [cfg (not (feature = "may_dangle"))] impl < T , const N : usize > Drop for SmallVec < T , N > { fn drop (& mut self) { let on_heap = self . spilled () ; let len = self . len () ; let ptr = self . as_mut_ptr () ; unsafe { let _drop_dealloc = if on_heap { let capacity = self . capacity () ; Some (DropDealloc { ptr : NonNull :: new_unchecked (ptr as * mut u8) , size_bytes : capacity * size_of :: < T > () , align : align_of :: < T > () , }) } else { None } ; core :: ptr :: slice_from_raw_parts_mut (ptr , len) . drop_in_place () ; } } }
    };
}

impl_119!()