// Generated macro for impl_513 (impl)
macro_rules! Depcrate_next_arrayimpl_513 {
() => {
// Module: crate::next_array
// Provides: {"impl_513"}
// Dependencies: {}
impl < T , const N : usize > ArrayBuilder < T , N > { # [doc = " Initializes a new, empty `ArrayBuilder`."] pub fn new () -> Self { Self { arr : [() ; N] . map (| _ | MaybeUninit :: uninit ()) , len : 0 , } } # [doc = " Pushes `value` onto the end of the array."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics if `self.len >= N`."] # [inline (always)] pub fn push (& mut self , value : T) { let place = & mut self . arr [self . len] ; * place = MaybeUninit :: new (value) ; self . len += 1 ; } # [doc = " Consumes the elements in the `ArrayBuilder` and returns them as an array"] # [doc = " `[T; N]`."] # [doc = ""] # [doc = " If `self.len() < N`, this returns `None`."] pub fn take (& mut self) -> Option < [T ; N] > { if self . len == N { self . len = 0 ; let arr = mem :: replace (& mut self . arr , [() ; N] . map (| _ | MaybeUninit :: uninit ())) ; Some (arr . map (| v | { unsafe { v . assume_init () } })) } else { None } } }
};
}
