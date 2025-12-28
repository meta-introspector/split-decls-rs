macro_rules! deps {
    () => {
        Gidx!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl Gidx { fn new (blocks : u32 , passes : u32 , segment_length : u32) -> Self { let mut block = [0u64 ; 128] ; block [1] = 0u64 ; block [3] = u64 :: from (blocks) ; block [4] = u64 :: from (passes) ; block [5] = u64 :: from (ARGON2_VARIANT) ; Self { block , addresses : [0u64 ; 128] , segment_length , offset : 0 , } } fn init (& mut self , pass_n : u32 , segment_n : u32 , offset : u32 , tmp_block : & mut [u64 ; 128]) { self . block [0] = u64 :: from (pass_n) ; self . block [2] = u64 :: from (segment_n) ; self . block [6] = 0u64 ; self . offset = offset ; self . next_addresses (tmp_block) ; } fn next_addresses (& mut self , tmp_block : & mut [u64 ; 128]) { self . block [6] += 1 ; tmp_block . copy_from_slice (& self . block) ; fill_block (tmp_block) ; xor_slices ! (self . block , tmp_block) ; self . addresses . copy_from_slice (tmp_block) ; fill_block (& mut self . addresses) ; xor_slices ! (tmp_block , self . addresses) ; } fn get_next (& mut self , segment_idx : u32 , tmp_block : & mut [u64 ; 128]) -> u32 { let j1 : u64 = self . addresses [self . offset as usize] & 0xFFFF_FFFFu64 ; self . offset = (self . offset + 1) % 128 ; if self . offset == 0 { self . next_addresses (tmp_block) ; } let n_blocks = self . block [3] as u32 ; let pass_n = self . block [0] as u32 ; let segment_n = self . block [2] as u32 ; let ref_start_pos : u32 = if pass_n == 0 && segment_n == 0 { segment_idx - 1 } else if pass_n == 0 { segment_n * self . segment_length + segment_idx - 1 } else { n_blocks - self . segment_length + segment_idx - 1 } ; let mut ref_pos : u64 = (j1 * j1) >> 32 ; ref_pos = (ref_start_pos as u64 * ref_pos) >> 32 ; ref_pos = (ref_start_pos as u64 - 1) - ref_pos ; if pass_n == 0 || segment_n == 3 { ref_pos as u32 % n_blocks } else { (self . segment_length * (segment_n + 1) + ref_pos as u32) % n_blocks } } }
    };
}

impl_320!();