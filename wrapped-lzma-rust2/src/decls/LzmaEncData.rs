macro_rules! LzmaEncData {
    () => {
        pub (crate) struct LzmaEncData { pub (crate) nice_len : usize , dist_price_count : i32 , align_price_count : i32 , dist_slot_prices_size : u32 , dist_slot_prices : Vec < Vec < u32 > > , full_dist_prices : [[u32 ; FULL_DISTANCES] ; DIST_STATES] , align_prices : [u32 ; ALIGN_SIZE] , pub (crate) back : i32 , pub (crate) read_ahead : i32 , pub (crate) uncompressed_size : u32 , }
    };
}

LzmaEncData!();