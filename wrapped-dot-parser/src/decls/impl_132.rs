macro_rules! deps {
    () => {
        NodeID!();
        EdgeRHS!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl EdgeRHS { fn get_node_ids (& self) -> HashSet < NodeID > { let mut nexts : HashSet < NodeID > = self . next . as_ref () . map (| n | n . get_node_ids ()) . unwrap_or_default () ; nexts . insert (self . to . clone ()) ; nexts } }
    };
}

impl_132!()