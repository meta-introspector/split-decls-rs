macro_rules! deps {
    () => {
        Merge!();
        IntoStream!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl < S , const N : usize > MergeTrait for [S ; N] where S : IntoStream , { type Item = < Merge < S :: IntoStream , N > as Stream > :: Item ; type Stream = Merge < S :: IntoStream , N > ; fn merge (self) -> Self :: Stream { Merge :: new (self . map (| i | i . into_stream ())) } }
    };
}

impl_438!()