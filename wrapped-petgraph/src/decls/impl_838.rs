macro_rules! deps {
    () => {
        NodeIndex!();
        Neighbors!();
        IndexType!();
    };
}

macro_rules! impl_838 {
    () => {
        deps!();
        impl < E , Ix > Iterator for Neighbors < '_ , E , Ix > where Ix : IndexType , { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < NodeIndex < Ix > > { match self . edges . get (self . next [0] . index ()) { None => { } Some (edge) => { debug_assert ! (edge . weight . is_some ()) ; self . next [0] = edge . next [0] ; return Some (edge . node [1]) ; } } while let Some (edge) = self . edges . get (self . next [1] . index ()) { debug_assert ! (edge . weight . is_some ()) ; self . next [1] = edge . next [1] ; if edge . node [0] != self . skip_start { return Some (edge . node [0]) ; } } None } }
    };
}

impl_838!()