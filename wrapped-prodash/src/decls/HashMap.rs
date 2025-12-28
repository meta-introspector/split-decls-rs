macro_rules! HashMap {
    () => {
        # [cfg (not (feature = "dashmap"))] type HashMap < K , V > = sync :: HashMap < K , V > ;
    };
}

HashMap!();