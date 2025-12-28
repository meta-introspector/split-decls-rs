macro_rules! deps {
    () => {
        ChunkState!();
        CVWords!();
        Output!();
    };
}

macro_rules! hash_all_at_once {
    () => {
        deps!();
        fn hash_all_at_once < J : join :: Join > (input : & [u8] , key : & CVWords , flags : u8) -> Output { let platform = Platform :: detect () ; if input . len () <= CHUNK_LEN { return ChunkState :: new (key , 0 , flags , platform) . update (input) . output () ; } Output { input_chaining_value : * key , block : compress_subtree_to_parent_node :: < J > (input , key , 0 , flags , platform) , block_len : BLOCK_LEN as u8 , counter : 0 , flags : flags | PARENT , platform , } }
    };
}

hash_all_at_once!()