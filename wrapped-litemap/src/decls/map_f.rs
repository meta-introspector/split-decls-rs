macro_rules! map_f {
    () => {
        # [inline] fn map_f < K , V > (input : & (K , V)) -> (& K , & V) { (& input . 0 , & input . 1) }
    };
}

map_f!();