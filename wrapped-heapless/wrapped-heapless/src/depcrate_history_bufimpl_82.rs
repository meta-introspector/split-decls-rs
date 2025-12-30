// Generated macro for impl_82 (impl)
macro_rules! Depcrate_history_bufimpl_82 {
() => {
// Module: crate::history_buf
// Provides: {"impl_82"}
// Dependencies: {}
impl < T , const N : usize > HistoryBuf < T , N > { const INIT : MaybeUninit < T > = MaybeUninit :: uninit () ; # [doc = " Constructs a new history buffer."] # [doc = ""] # [doc = " The construction of a `HistoryBuf` works in `const` contexts."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::HistoryBuf;"] # [doc = ""] # [doc = " // Allocate a 16-element buffer on the stack"] # [doc = " let x: HistoryBuf<u8, 16> = HistoryBuf::new();"] # [doc = " assert_eq!(x.len(), 0);"] # [doc = " ```"] # [inline] pub const fn new () -> Self { const { assert ! (N > 0) ; } Self { phantom : PhantomData , data : HistoryBufStorageInner { buffer : [Self :: INIT ; N] , } , write_at : 0 , filled : false , } } }
};
}
