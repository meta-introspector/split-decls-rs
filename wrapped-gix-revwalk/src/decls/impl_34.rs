macro_rules! deps {
    () => {
        PriorityQueue!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < K , T > Clone for PriorityQueue < K , T > where K : Clone + Ord , T : Clone , { fn clone (& self) -> Self { Self (self . 0 . clone ()) } }
    };
}

impl_34!()