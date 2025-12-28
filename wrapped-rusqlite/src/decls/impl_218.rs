macro_rules! deps {
    () => {
        MappedRows!();
        Row!();
        Result!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < T , F > Iterator for MappedRows < '_ , F > where F : FnMut (& Row < '_ >) -> Result < T > , { type Item = Result < T > ; # [inline] fn next (& mut self) -> Option < Result < T > > { let map = & mut self . map ; self . rows . next () . transpose () . map (| row_result | row_result . and_then (map)) } }
    };
}

impl_218!();