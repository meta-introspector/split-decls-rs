macro_rules! array_assume_init {
    () => {
        # [doc = " Extracts the values from an array of `MaybeUninit` containers."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It is up to the caller to guarantee that all elements of the array are"] # [doc = " in an initialized state."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #![feature(maybe_uninit_uninit_array)]"] # [doc = " #![feature(maybe_uninit_array_assume_init)]"] # [doc = " use std::mem::MaybeUninit;"] # [doc = ""] # [doc = " let mut array: [MaybeUninit<i32>; 3] = MaybeUninit::uninit_array();"] # [doc = " array[0].write(0);"] # [doc = " array[1].write(1);"] # [doc = " array[2].write(2);"] # [doc = ""] # [doc = " // SAFETY: Now safe as we initialised all elements"] # [doc = " let array = unsafe {"] # [doc = "     MaybeUninit::array_assume_init(array)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(array, [0, 1, 2]);"] # [doc = " ```"] # [inline (always)] pub unsafe fn array_assume_init < T , const N : usize > (array : [MaybeUninit < T > ; N]) -> [T ; N] { unsafe { (& array as * const _ as * const [T ; N]) . read () } }
    };
}

array_assume_init!();