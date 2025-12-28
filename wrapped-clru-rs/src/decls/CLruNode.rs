macro_rules! CLruNode {
    () => {
        # [derive (Debug)] struct CLruNode < K , V > { key : K , value : V , }
    };
}

CLruNode!();