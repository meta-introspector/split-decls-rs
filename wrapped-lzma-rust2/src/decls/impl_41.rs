macro_rules! deps {
    () => {
        MfType!();
        LzEncoder!();
        MatchFinders!();
        Matches!();
        LzEncoderData!();
        Hc4!();
        Bt4!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl LzEncoder { pub (crate) fn get_memory_usage (dict_size : u32 , extra_size_before : u32 , extra_size_after : u32 , match_len_max : u32 , mf : MfType ,) -> u32 { get_buf_size (dict_size , extra_size_before , extra_size_after , match_len_max ,) + mf . get_memory_usage (dict_size) } pub (crate) fn new_hc4 (dict_size : u32 , extra_size_before : u32 , extra_size_after : u32 , nice_len : u32 , match_len_max : u32 , depth_limit : i32 ,) -> Self { Self :: new (dict_size , extra_size_before , extra_size_after , nice_len , match_len_max , MatchFinders :: Hc4 (Hc4 :: new (dict_size , nice_len , depth_limit)) ,) } pub (crate) fn new_bt4 (dict_size : u32 , extra_size_before : u32 , extra_size_after : u32 , nice_len : u32 , match_len_max : u32 , depth_limit : i32 ,) -> Self { Self :: new (dict_size , extra_size_before , extra_size_after , nice_len , match_len_max , MatchFinders :: Bt4 (Bt4 :: new (dict_size , nice_len , depth_limit)) ,) } fn new (dict_size : u32 , extra_size_before : u32 , extra_size_after : u32 , nice_len : u32 , match_len_max : u32 , match_finder : MatchFinders ,) -> Self { let buf_size = get_buf_size (dict_size , extra_size_before , extra_size_after , match_len_max ,) ; let buf_size = buf_size as usize ; let buf = vec ! [0 ; buf_size] ; let buf_limit_u16 = buf_size . checked_sub (size_of :: < u16 > ()) . unwrap () ; let keep_size_before = extra_size_before + dict_size ; let keep_size_after = extra_size_after + match_len_max ; Self { data : LzEncoderData { keep_size_before , keep_size_after , match_len_max , nice_len , buf , buf_size , buf_limit_u16 , read_pos : - 1 , read_limit : - 1 , finishing : false , write_pos : 0 , pending_size : 0 , } , matches : Matches :: new (nice_len as usize - 1) , match_finder , } } pub (crate) fn normalize (positions : & mut [i32] , norm_offset : i32) { # [cfg (all (feature = "std" , feature = "optimization" , target_arch = "x86_64"))] { if std :: arch :: is_x86_feature_detected ! ("avx2") { return unsafe { normalize_avx2 (positions , norm_offset) } ; } if std :: arch :: is_x86_feature_detected ! ("sse4.1") { return unsafe { normalize_sse41 (positions , norm_offset) } ; } } # [cfg (all (feature = "std" , feature = "optimization" , target_arch = "aarch64"))] { if std :: arch :: is_aarch64_feature_detected ! ("neon") { return unsafe { normalize_neon (positions , norm_offset) } ; } } normalize_scalar (positions , norm_offset) ; } pub (crate) fn find_matches (& mut self) { self . match_finder . find_matches (& mut self . data , & mut self . matches) } pub (crate) fn matches (& mut self) -> & mut Matches { & mut self . matches } pub (crate) fn skip (& mut self , len : usize) { self . match_finder . skip (& mut self . data , len) } pub (crate) fn set_preset_dict (& mut self , dict_size : u32 , preset_dict : & [u8]) { self . data . set_preset_dict (dict_size , preset_dict , & mut self . match_finder) } pub (crate) fn set_finishing (& mut self) { self . data . set_finishing (& mut self . match_finder) } pub (crate) fn fill_window (& mut self , input : & [u8]) -> usize { self . data . fill_window (input , & mut self . match_finder) } pub (crate) fn set_flushing (& mut self) { self . data . set_flushing (& mut self . match_finder) } pub (crate) fn verify_matches (& self) -> bool { self . data . verify_matches (& self . matches) } }
    };
}

impl_41!()