macro_rules! update_crc_with_padding {
    () => {
        fn update_crc_with_padding (crc : & mut crc :: Digest < '_ , u32 > , padding_needed : usize) { match padding_needed { 1 => crc . update (& [0]) , 2 => crc . update (& [0 , 0]) , 3 => crc . update (& [0 , 0 , 0]) , _ => { } } }
    };
}

update_crc_with_padding!()