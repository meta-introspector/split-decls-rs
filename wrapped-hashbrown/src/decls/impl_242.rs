macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl < K , V , S , A > Eq for HashMap < K , V , S , A > where K : Eq + Hash , V : Eq , S : BuildHasher , A : Allocator , { }
    };
}

impl_242!()