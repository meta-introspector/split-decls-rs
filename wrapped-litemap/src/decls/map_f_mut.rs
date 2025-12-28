macro_rules! map_f_mut {
    () => {
        # [inline] fn map_f_mut < K , V > (input : & mut (K , V)) -> (& K , & mut V) { (& input . 0 , & mut input . 1) }
    };
}

map_f_mut!();