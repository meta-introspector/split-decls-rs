macro_rules! Node {
    () => {
        struct Node < K , V > { entry : MaybeUninit < (K , V) > , links : Links < K , V > , }
    };
}

Node!()