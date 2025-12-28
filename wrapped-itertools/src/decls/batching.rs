macro_rules! deps {
    () => {
        Batching!();
    };
}

macro_rules! batching {
    () => {
        deps!();
        # [doc = " Create a new Batching iterator."] pub fn batching < I , F > (iter : I , f : F) -> Batching < I , F > { Batching { f , iter } }
    };
}

batching!()