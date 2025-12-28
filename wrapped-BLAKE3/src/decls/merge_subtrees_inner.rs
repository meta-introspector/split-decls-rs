macro_rules! deps {
    () => {
        Platform!();
        Output!();
        ChainingValue!();
        Mode!();
    };
}

macro_rules! merge_subtrees_inner {
    () => {
        deps!();
        fn merge_subtrees_inner (left_child : & ChainingValue , right_child : & ChainingValue , mode : Mode ,) -> crate :: Output { crate :: parent_node_output (& left_child , & right_child , & mode . key_words () , mode . flags_byte () , Platform :: detect () ,) }
    };
}

merge_subtrees_inner!();