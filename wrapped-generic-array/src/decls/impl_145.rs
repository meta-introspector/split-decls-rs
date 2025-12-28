macro_rules! deps {
    () => {
        ArrayBuilder!();
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < T , N : ArrayLength > ArrayBuilder < T , N > { # [doc = " Begin building an array"] # [inline (always)] pub const fn new () -> ArrayBuilder < T , N > { ArrayBuilder { array : GenericArray :: uninit () , position : 0 , } } # [doc = " Consume an iterator, `.zip`-ing it to fill some or all of the array. This does not check if the"] # [doc = " iterator had extra elements or too few elements."] # [doc = ""] # [doc = " This makes no attempt to continue where a previous `extend` leaves off. Therefore, it should"] # [doc = " only be used once per `ArrayBuilder`."] # [inline (always)] pub unsafe fn extend (& mut self , source : impl Iterator < Item = T >) { let (destination , position) = (self . array . iter_mut () , & mut self . position) ; destination . zip (source) . for_each (| (dst , src) | { dst . write (src) ; * position += 1 ; }) ; } # [doc = " Returns true if the write position equals the array size"] # [inline (always)] pub const fn is_full (& self) -> bool { self . position == N :: USIZE } # [doc = " Creates a mutable iterator for writing to the array elements."] # [doc = ""] # [doc = " You MUST increment the position value (given as a mutable reference) as you iterate"] # [doc = " to mark how many elements have been created."] # [doc = ""] # [doc = " ```"] # [doc = " #[cfg(feature = \"internals\")]"] # [doc = " # {"] # [doc = " # use generic_array::{GenericArray, internals::ArrayBuilder, typenum::U5};"] # [doc = " # struct SomeType;"] # [doc = " fn make_some_struct() -> SomeType { SomeType }"] # [doc = " unsafe {"] # [doc = "     let mut builder = ArrayBuilder::<SomeType, U5>::new();"] # [doc = "     let (dst_iter, position) = builder.iter_position();"] # [doc = "     for dst in dst_iter {"] # [doc = "         dst.write(make_some_struct());"] # [doc = "         // MUST be done AFTER ownership of the value has been given to `dst.write`"] # [doc = "         *position += 1;"] # [doc = "     }"] # [doc = "     let your_array = builder.assume_init();"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] # [inline (always)] pub unsafe fn iter_position (& '_ mut self ,) -> (slice :: IterMut < '_ , MaybeUninit < T > > , & '_ mut usize) { (self . array . iter_mut () , & mut self . position) } # [doc = " When done writing (assuming all elements have been written to),"] # [doc = " get the inner array."] # [doc = ""] # [doc = " This method is `const` since Rust 1.83.0, but non-`const` before."] # [rustversion :: attr (since (1.83) , const)] # [inline (always)] pub unsafe fn assume_init (self) -> GenericArray < T , N > { debug_assert ! (self . is_full ()) ; let array = ptr :: read (& self . array) ; mem :: forget (self) ; GenericArray :: assume_init (array) } }
    };
}

impl_145!()