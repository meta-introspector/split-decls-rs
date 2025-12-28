macro_rules! deps {
    () => {
        Bt4!();
        LzEncoderData!();
        Hash234!();
        LzEncoder!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Bt4 { pub (crate) fn new (dict_size : u32 , nice_len : u32 , depth_limit : i32) -> Self { let cyclic_size = dict_size as i32 + 1 ; let tree = vec ! [0 ; cyclic_size as usize * 2] ; Self { hash : Hash234 :: new (dict_size) , tree , depth_limit : if depth_limit > 0 { depth_limit } else { 16 + nice_len as i32 / 2 } , cyclic_size , cyclic_pos : - 1 , lz_pos : cyclic_size , } } pub (crate) fn get_mem_usage (dict_size : u32) -> u32 { Hash234 :: get_mem_usage (dict_size) + dict_size / (1024 / 8) + 10 } fn move_pos (& mut self , encoder : & mut super :: LzEncoderData) -> i32 { let avail = encoder . move_pos (encoder . nice_len as _ , 4) ; if avail != 0 { self . lz_pos += 1 ; if self . lz_pos == MAX_POS { let normalization_offset = MAX_POS - self . cyclic_size ; self . hash . normalize (normalization_offset) ; LzEncoder :: normalize (& mut self . tree , normalization_offset) ; self . lz_pos -= normalization_offset ; } self . cyclic_pos += 1 ; if self . cyclic_pos == self . cyclic_size { self . cyclic_pos = 0 ; } } avail } fn skip (& mut self , encoder : & mut super :: LzEncoderData , nice_len_limit : i32 , mut current_match : i32 ,) { let mut depth = self . depth_limit ; let mut ptr0 = sh_left (self . cyclic_pos) + 1 ; let mut ptr1 = sh_left (self . cyclic_pos) ; let mut len0 = 0 ; let mut len1 = 0 ; loop { let delta = self . lz_pos - current_match ; if depth == 0 || delta >= self . cyclic_size { self . tree [ptr0 as usize] = 0 ; self . tree [ptr1 as usize] = 0 ; return ; } depth -= 1 ; let pair_selector = self . cyclic_size * ((delta > self . cyclic_pos) as i32) ; let pair = sh_left (self . cyclic_pos - delta + pair_selector) ; let mut len = len0 . min (len1) ; if encoder . get_byte_by_pos (encoder . read_pos + len - delta) == encoder . get_byte_by_pos (encoder . read_pos + len) { loop { len += 1 ; if len == nice_len_limit { self . tree [ptr1 as usize] = self . tree [pair as usize] ; self . tree [ptr0 as usize] = self . tree [pair as usize + 1] ; return ; } if encoder . get_byte (len as _ , delta as _) != encoder . get_byte (len as _ , 0) { break ; } } } if encoder . get_byte (len as _ , delta) < encoder . get_byte (len as _ , 0) { self . tree [ptr1 as usize] = current_match ; ptr1 = pair + 1 ; current_match = self . tree [ptr1 as usize] ; len1 = len ; } else { self . tree [ptr0 as usize] = current_match ; ptr0 = pair ; current_match = self . tree [ptr0 as usize] ; len0 = len ; } } } }
    };
}

impl_12!()