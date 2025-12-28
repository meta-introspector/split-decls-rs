macro_rules! deps {
    () => {
        StreamDeserializer!();
    };
}

macro_rules! Fused {
    () => {
        deps!();
        # [doc = " Marker for whether StreamDeserializer can implement FusedIterator."] pub trait Fused : private :: Sealed { }
    };
}

Fused!()