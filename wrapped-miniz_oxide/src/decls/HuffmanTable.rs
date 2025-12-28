macro_rules! deps {
    () => {
        BigArray!();
    };
}

macro_rules! HuffmanTable {
    () => {
        deps!();
        # [doc = " A struct containing huffman code lengths and the huffman code tree used by the decompressor."] # [cfg_attr (not (feature = "rustc-dep-of-std") , derive (Clone))] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] struct HuffmanTable { # [doc = " Fast lookup table for shorter huffman codes."] # [doc = ""] # [doc = " See `HuffmanTable::fast_lookup`."] # [cfg_attr (feature = "serde" , serde (with = "BigArray"))] pub look_up : [i16 ; FAST_LOOKUP_SIZE as usize] , # [doc = " Full huffman tree."] # [doc = ""] # [doc = " Positive values are edge nodes/symbols, negative values are"] # [doc = " parent nodes/references to other nodes."] # [cfg_attr (feature = "serde" , serde (with = "BigArray"))] pub tree : [i16 ; MAX_HUFF_TREE_SIZE] , }
    };
}

HuffmanTable!()