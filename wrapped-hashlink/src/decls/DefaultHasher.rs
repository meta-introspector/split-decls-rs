macro_rules! deps {
    () => {
        DefaultHashBuilder!();
    };
}

macro_rules! DefaultHasher {
    () => {
        deps!();
        # [doc = " Default hasher, as selected by hashbrown."] # [derive (Clone)] pub struct DefaultHasher (< hashbrown :: DefaultHashBuilder as BuildHasher > :: Hasher) ;
    };
}

DefaultHasher!()