macro_rules! array_assume_init {
    () => {
        unsafe fn array_assume_init < T , const N : usize > (array : [MaybeUninit < T > ; N]) -> [T ; N] { let ret = unsafe { (& array as * const _ as * const [T ; N]) . read () } ; # [allow (clippy :: forget_non_drop)] mem :: forget (array) ; ret }
    };
}

array_assume_init!()