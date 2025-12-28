macro_rules! deps {
    () => {
        HashValue!();
    };
}

macro_rules! Bucket {
    () => {
        deps!();
        # [derive (Copy , Debug)] struct Bucket < K , V > { hash : HashValue , key : K , value : V , }
    };
}

Bucket!();