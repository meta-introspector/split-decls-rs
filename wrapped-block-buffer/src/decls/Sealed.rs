macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! Sealed {
    () => {
        deps!();
        # [doc = " Sealed trait for buffer kinds."] pub trait Sealed { # [cfg (not (feature = "zeroize"))] type Pos : Default + Clone ; # [cfg (feature = "zeroize")] type Pos : Default + Clone + zeroize :: Zeroize ; type Overhead : ArraySize ; const NAME : & 'static str ; fn get_pos < N : ArraySize > (buf : & Block < N > , pos : & Self :: Pos) -> usize ; fn set_pos < N : ArraySize > (buf : & mut Block < N > , pos : & mut Self :: Pos , val : usize) ; # [doc = " Invariant guaranteed by a buffer kind, i.e. with correct"] # [doc = " buffer code this function always returns true."] fn invariant (pos : usize , block_size : usize) -> bool ; # [doc = " Split input data into slice of blocks and tail."] fn split_blocks < N : ArraySize > (data : & [u8]) -> (& [Array < u8 , N >] , & [u8]) ; }
    };
}

Sealed!()