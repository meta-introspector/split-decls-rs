// Generated macro for impl_83 (impl)
macro_rules! Depcrate_history_bufimpl_83 {
() => {
// Module: crate::history_buf
// Provides: {"impl_83"}
// Dependencies: {}
impl < T , const N : usize > HistoryBuf < T , N > where T : Copy + Clone , { # [doc = " Constructs a new history buffer, where every element is the given value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::HistoryBuf;"] # [doc = ""] # [doc = " // Allocate a 16-element buffer on the stack"] # [doc = " let mut x: HistoryBuf<u8, 16> = HistoryBuf::new_with(4);"] # [doc = " // All elements are four"] # [doc = " assert_eq!(x.as_slice(), [4; 16]);"] # [doc = " ```"] # [inline] pub fn new_with (t : T) -> Self { Self { phantom : PhantomData , data : HistoryBufStorageInner { buffer : [MaybeUninit :: new (t) ; N] , } , write_at : 0 , filled : true , } } }
};
}
