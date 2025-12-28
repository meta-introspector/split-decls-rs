macro_rules! deps {
    () => {
        IndentHeuristic!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < IndentOfToken > IndentHeuristic < IndentOfToken > { pub fn new (indent_of_token : IndentOfToken) -> Self { Self { indent_of_token } } }
    };
}

impl_39!()