macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! macro_275 {
    () => {
        deps!();
        serde_impl ! (Bytes , BytesVisitor , copy_from_slice , from) ;
    };
}

macro_275!();