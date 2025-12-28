macro_rules! deps {
    () => {
        Create!();
        NodeFiltered!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < F , G > NodeFiltered < G , F > where G : GraphBase , F : Fn (G :: NodeId) -> bool , { # [doc = " Create an `NodeFiltered` adaptor from the closure `filter`."] pub fn from_fn (graph : G , filter : F) -> Self { NodeFiltered (graph , filter) } }
    };
}

impl_118!();