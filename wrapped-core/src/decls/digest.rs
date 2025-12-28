macro_rules! deps {
    () => {
        Digest!();
        Blocks!();
    };
}

macro_rules! digest {
    () => {
        deps!();
        const fn digest (mut state : [u32 ; 5] , len : u64 , blocks : Blocks) -> Digest { const fn clone_from_slice_128 (mut data : [u8 ; 128] , slice : & [u8] , offset : usize , num_elems : usize ,) -> [u8 ; 128] { let mut i = 0 ; while i < num_elems { data [i] = slice [offset + i] ; i += 1 ; } data } const fn clone_slice_128 (mut data : [u8 ; 128] , slice : & [u8] , _offset : usize) -> [u8 ; 128] { let mut i = 0 ; while i < slice . len () { data [_offset + i] = slice [i] ; i += 1 ; } data } const fn as_block (input : & [u8] , offset : usize) -> [u32 ; 16] { let mut result = [0u32 ; 16] ; let mut i = 0 ; while i != 16 { let off = offset + (i * 4) ; result [i] = (input [off + 3] as u32) | ((input [off + 2] as u32) << 8) | ((input [off + 1] as u32) << 16) | ((input [off] as u32) << 24) ; i += 1 ; } result } let bits = (len + (blocks . len as u64)) * 8 ; let extra = [(bits >> 56) as u8 , (bits >> 48) as u8 , (bits >> 40) as u8 , (bits >> 32) as u8 , (bits >> 24) as u8 , (bits >> 16) as u8 , (bits >> 8) as u8 , bits as u8 ,] ; let mut last = [0 ; 128] ; let blocklen = blocks . len as usize ; last = clone_from_slice_128 (last , & blocks . data , 0 , blocklen) ; last [blocklen] = 0x80 ; if blocklen < 56 { last = clone_slice_128 (last , & extra , 56) ; state = process_state (state , as_block (& last , 0)) ; } else { last = clone_slice_128 (last , & extra , 120) ; state = process_state (state , as_block (& last , 0)) ; state = process_state (state , as_block (& last , 64)) ; } Digest { data : state } }
    };
}

digest!();