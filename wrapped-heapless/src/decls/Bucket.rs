macro_rules! deps {
    () => {
        HashValue!();
    };
}

macro_rules! Bucket {
    () => {
        deps!();
        # [doc (hidden)] # [derive (Clone)] # [cfg_attr (feature = "zeroize" , derive (Zeroize))] pub struct Bucket < K , V > { hash : HashValue , key : K , value : V , }
    };
}

Bucket!();