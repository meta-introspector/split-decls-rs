// Generated macro for Builder (struct)
macro_rules! DepcrateBuilder {
() => {
// Module: crate
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " IoUring build params"] # [derive (Clone , Default)] pub struct Builder < S = squeue :: Entry , C = cqueue :: Entry > where S : squeue :: EntryMarker , C : cqueue :: EntryMarker , { dontfork : bool , params : sys :: io_uring_params , phantom : PhantomData < (S , C) > , }
};
}
