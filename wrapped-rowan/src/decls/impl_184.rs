macro_rules! deps {
    () => {
        AstChildren!();
        AstNode!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < N : AstNode > Iterator for AstChildren < N > { type Item = N ; fn next (& mut self) -> Option < N > { self . inner . find_map (N :: cast) } }
    };
}

impl_184!()