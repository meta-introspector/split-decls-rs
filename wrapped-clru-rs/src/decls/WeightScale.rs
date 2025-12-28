macro_rules! WeightScale {
    () => {
        # [doc = " Trait used to retrieve the weight of a key-value pair."] pub trait WeightScale < K , V > { # [doc = " Returns the weight of a key-value pair."] fn weight (& self , key : & K , value : & V) -> usize ; }
    };
}

WeightScale!();