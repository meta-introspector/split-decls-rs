macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Any { pub (crate) unsafe fn new < T > (t : T) -> Self { let value : Value ; let drop : unsafe fn (& mut Value) ; let type_id = typeid :: of :: < T > () ; if is_small :: < T > () { let mut inline = [MaybeUninit :: uninit () ; 2] ; unsafe { ptr :: write (inline . as_mut_ptr () . cast :: < T > () , t) } ; value = Value { inline } ; unsafe fn inline_drop < T > (value : & mut Value) { unsafe { ptr :: drop_in_place (value . inline . as_mut_ptr () . cast :: < T > ()) } } drop = inline_drop :: < T > ; } else { let ptr = Box :: into_raw (Box :: new (t)) . cast :: < () > () ; value = Value { ptr } ; unsafe fn ptr_drop < T > (value : & mut Value) { mem :: drop (unsafe { Box :: from_raw (value . ptr . cast :: < T > ()) }) ; } drop = ptr_drop :: < T > ; } Any { value , drop , type_id , # [cfg (feature = "unstable-debug")] type_name : any :: type_name :: < T > () , } } pub (crate) unsafe fn take < T > (mut self) -> T { if self . type_id != typeid :: of :: < T > () { self . invalid_cast_to :: < T > () ; } if is_small :: < T > () { let ptr = unsafe { self . value . inline . as_mut_ptr () . cast :: < T > () } ; let value = unsafe { ptr :: read (ptr) } ; mem :: forget (self) ; value } else { let ptr = unsafe { self . value . ptr . cast :: < T > () } ; let box_t = unsafe { Box :: from_raw (ptr) } ; mem :: forget (self) ; * box_t } } # [cfg (not (feature = "unstable-debug"))] fn invalid_cast_to < T > (& self) -> ! { panic ! ("invalid cast; enable `unstable-debug` feature to debug") ; } # [cfg (feature = "unstable-debug")] fn invalid_cast_to < T > (& self) -> ! { let from = self . type_name ; let to = any :: type_name :: < T > () ; panic ! ("invalid cast: {} to {}" , from , to) ; } }
    };
}

impl_11!()