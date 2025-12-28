macro_rules! deps {
    () => {
        EdgeReferences!();
        IndexType!();
        EdgeReference!();
    };
}

macro_rules! impl_832 {
    () => {
        deps!();
        impl < 'a , E , Ix > Iterator for EdgeReferences < 'a , E , Ix > where Ix : IndexType , { type Item = EdgeReference < 'a , E , Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . ex_find_map (| (i , edge) | { edge . weight . as_ref () . map (move | weight | EdgeReference { index : edge_index (i) , node : edge . node , weight , }) }) } }
    };
}

impl_832!();