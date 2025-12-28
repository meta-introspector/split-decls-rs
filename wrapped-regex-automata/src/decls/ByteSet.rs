macro_rules! ByteSet {
    () => {
        # [derive (Clone , Debug)] pub (crate) struct ByteSet ([bool ; 256]) ;
    };
}

ByteSet!()