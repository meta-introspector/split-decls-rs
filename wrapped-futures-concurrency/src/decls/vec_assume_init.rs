macro_rules! vec_assume_init {
    () => {
        unsafe fn vec_assume_init < T > (vec : Vec < MaybeUninit < T > >) -> Vec < T > { let ret = unsafe { (& vec as * const _ as * const Vec < T >) . read () } ; mem :: forget (vec) ; ret }
    };
}

vec_assume_init!();