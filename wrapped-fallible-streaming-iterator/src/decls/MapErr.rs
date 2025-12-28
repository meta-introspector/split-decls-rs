macro_rules! MapErr {
    () => {
        # [doc = " An iterator which applies a transform to errors."] pub struct MapErr < I , F > { it : I , f : F , }
    };
}

MapErr!()