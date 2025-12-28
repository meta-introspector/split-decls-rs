macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < T , N : ArrayLength > GenericArray < T , N > { # [doc = " Create a new array of `MaybeUninit<T>` items, in an uninitialized state."] # [doc = ""] # [doc = " See [`GenericArray::assume_init`] for a full example."] # [inline (always)] # [allow (clippy :: uninit_assumed_init)] pub const fn uninit () -> GenericArray < MaybeUninit < T > , N > { unsafe { MaybeUninit :: < GenericArray < MaybeUninit < T > , N > > :: uninit () . assume_init () } } # [doc = " Extracts the values from a generic array of `MaybeUninit` containers."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It is up to the caller to guarantee that all elements of the array are in an initialized state."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use core::mem::MaybeUninit;"] # [doc = " # use generic_array::{GenericArray, typenum::U3, arr};"] # [doc = " let mut array: GenericArray<MaybeUninit<i32>, U3> = GenericArray::uninit();"] # [doc = " array[0].write(0);"] # [doc = " array[1].write(1);"] # [doc = " array[2].write(2);"] # [doc = ""] # [doc = " // SAFETY: Now safe as we initialised all elements"] # [doc = " let array = unsafe {"] # [doc = "     GenericArray::assume_init(array)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(array, arr![0, 1, 2]);"] # [doc = " ```"] # [inline (always)] pub const unsafe fn assume_init (array : GenericArray < MaybeUninit < T > , N >) -> Self { const_transmute :: < GenericArray < MaybeUninit < T > , N > , GenericArray < T , N > > (array) } }
    };
}

impl_192!()