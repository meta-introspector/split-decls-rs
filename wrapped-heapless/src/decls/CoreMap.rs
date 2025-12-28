macro_rules! deps {
    () => {
        Vec!();
        Pos!();
        Bucket!();
    };
}

macro_rules! CoreMap {
    () => {
        deps!();
        # [cfg_attr (feature = "zeroize" , derive (Zeroize) , zeroize (bound = "K: Zeroize, V: Zeroize"))] struct CoreMap < K , V , const N : usize > { entries : Vec < Bucket < K , V > , N , usize > , indices : [Option < Pos > ; N] , }
    };
}

CoreMap!();