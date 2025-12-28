macro_rules! deps {
    () => {
        Csr!();
    };
}

macro_rules! CsrError {
    () => {
        deps!();
        # [doc = " The error type for fallible operations with `Csr`."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum CsrError { # [doc = " Both vertex indexes go outside the graph."] IndicesOutBounds (usize , usize) , }
    };
}

CsrError!()