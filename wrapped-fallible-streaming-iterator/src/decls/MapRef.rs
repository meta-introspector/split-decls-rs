macro_rules! MapRef {
    () => {
        # [doc = " An iterator which applies a transform to elements."] pub struct MapRef < I , F > { it : I , f : F , }
    };
}

MapRef!()