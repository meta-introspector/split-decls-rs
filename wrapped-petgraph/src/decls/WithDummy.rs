macro_rules! WithDummy {
    () => {
        trait WithDummy : NodeIndexable { fn dummy_idx (& self) -> usize ; # [doc = " Convert `i` to a node index, returns None for the dummy node"] fn try_from_index (& self , i : usize) -> Option < Self :: NodeId > ; }
    };
}

WithDummy!();