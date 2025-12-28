macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! macro_276 {
    () => {
        deps!();
        serde_impl ! (BytesMut , BytesMutVisitor , from , from_vec) ;
    };
}

macro_276!()