macro_rules! deps {
    () => {
        Chain!();
    };
}

macro_rules! ChainSeq {
    () => {
        deps!();
        # [doc = " Wrapper for `Chain` to implement `ExactSizeIterator`"] struct ChainSeq < A , B > { chain : iter :: Chain < A , B > , }
    };
}

ChainSeq!()