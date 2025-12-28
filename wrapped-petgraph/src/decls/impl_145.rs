macro_rules! deps {
    () => {
        Create!();
        EdgeRef!();
        EdgeFiltered!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < F , G > EdgeFiltered < G , F > where G : IntoEdgeReferences , F : Fn (G :: EdgeRef) -> bool , { # [doc = " Create an `EdgeFiltered` adaptor from the closure `filter`."] pub fn from_fn (graph : G , filter : F) -> Self { EdgeFiltered (graph , filter) } }
    };
}

impl_145!()