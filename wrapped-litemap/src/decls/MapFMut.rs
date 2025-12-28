macro_rules! MapFMut {
    () => {
        type MapFMut < K , V > = fn (& mut (K , V)) -> (& K , & mut V) ;
    };
}

MapFMut!();