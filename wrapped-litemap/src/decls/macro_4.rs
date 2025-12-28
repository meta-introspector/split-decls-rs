macro_rules! macro_4 {
    () => {
        litemap_impl ! (feature = "alloc" , S = alloc :: vec :: Vec < (K , V) >) ;
    };
}

macro_4!()