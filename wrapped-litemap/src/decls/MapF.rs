macro_rules! MapF {
    () => {
        type MapF < K , V > = fn (& (K , V)) -> (& K , & V) ;
    };
}

MapF!()