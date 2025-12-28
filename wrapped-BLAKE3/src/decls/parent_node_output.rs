macro_rules! deps {
    () => {
        CVWords!();
        Output!();
        CVBytes!();
    };
}

macro_rules! parent_node_output {
    () => {
        deps!();
        fn parent_node_output (left_child : & CVBytes , right_child : & CVBytes , key : & CVWords , flags : u8 , platform : Platform ,) -> Output { let mut block = [0 ; BLOCK_LEN] ; block [.. 32] . copy_from_slice (left_child) ; block [32 ..] . copy_from_slice (right_child) ; Output { input_chaining_value : * key , block , block_len : BLOCK_LEN as u8 , counter : 0 , flags : flags | PARENT , platform , } }
    };
}

parent_node_output!()