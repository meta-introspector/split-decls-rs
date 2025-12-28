macro_rules! deps {
    () => {
        Platform!();
        ChunkState!();
        CVWords!();
        Output!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl ChunkState { fn new (key : & CVWords , chunk_counter : u64 , flags : u8 , platform : Platform) -> Self { Self { cv : * key , chunk_counter , buf : [0 ; BLOCK_LEN] , buf_len : 0 , blocks_compressed : 0 , flags , platform , } } fn count (& self) -> usize { BLOCK_LEN * self . blocks_compressed as usize + self . buf_len as usize } fn fill_buf (& mut self , input : & mut & [u8]) { let want = BLOCK_LEN - self . buf_len as usize ; let take = cmp :: min (want , input . len ()) ; self . buf [self . buf_len as usize ..] [.. take] . copy_from_slice (& input [.. take]) ; self . buf_len += take as u8 ; * input = & input [take ..] ; } fn start_flag (& self) -> u8 { if self . blocks_compressed == 0 { CHUNK_START } else { 0 } } fn update (& mut self , mut input : & [u8]) -> & mut Self { if self . buf_len > 0 { self . fill_buf (& mut input) ; if ! input . is_empty () { debug_assert_eq ! (self . buf_len as usize , BLOCK_LEN) ; let block_flags = self . flags | self . start_flag () ; self . platform . compress_in_place (& mut self . cv , & self . buf , BLOCK_LEN as u8 , self . chunk_counter , block_flags ,) ; self . buf_len = 0 ; self . buf = [0 ; BLOCK_LEN] ; self . blocks_compressed += 1 ; } } while input . len () > BLOCK_LEN { debug_assert_eq ! (self . buf_len , 0) ; let block_flags = self . flags | self . start_flag () ; self . platform . compress_in_place (& mut self . cv , array_ref ! (input , 0 , BLOCK_LEN) , BLOCK_LEN as u8 , self . chunk_counter , block_flags ,) ; self . blocks_compressed += 1 ; input = & input [BLOCK_LEN ..] ; } self . fill_buf (& mut input) ; debug_assert ! (input . is_empty ()) ; debug_assert ! (self . count () <= CHUNK_LEN) ; self } fn output (& self) -> Output { let block_flags = self . flags | self . start_flag () | CHUNK_END ; Output { input_chaining_value : self . cv , block : self . buf , block_len : self . buf_len , counter : self . chunk_counter , flags : block_flags , platform : self . platform , } } }
    };
}

impl_187!();