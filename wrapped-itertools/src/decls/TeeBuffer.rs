macro_rules! TeeBuffer {
    () => {
        # [doc = " Common buffer object for the two tee halves"] # [derive (Debug)] struct TeeBuffer < A , I > { backlog : VecDeque < A > , iter : I , # [doc = " The owner field indicates which id should read from the backlog"] owner : bool , }
    };
}

TeeBuffer!();