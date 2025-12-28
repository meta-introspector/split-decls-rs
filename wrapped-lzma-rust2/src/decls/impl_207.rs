macro_rules! deps {
    () => {
        MfType!();
        LzmaOptions!();
        Bt4!();
        LzmaEncoder!();
        EncodeMode!();
        Hc4!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl LzmaOptions { # [doc = " Default number of literal context bits."] pub const LC_DEFAULT : u32 = 3 ; # [doc = " Default number of literal position bits."] pub const LP_DEFAULT : u32 = 0 ; # [doc = " Default number of position bits."] pub const PB_DEFAULT : u32 = 2 ; # [doc = " Maximum match finder nice length."] pub const NICE_LEN_MAX : u32 = 273 ; # [doc = " Minimum match finder nice length."] pub const NICE_LEN_MIN : u32 = 8 ; # [doc = " Default dictionary size (8MB)."] pub const DICT_SIZE_DEFAULT : u32 = 8 << 20 ; const PRESET_TO_DICT_SIZE : & 'static [u32] = & [1 << 18 , 1 << 20 , 1 << 21 , 1 << 22 , 1 << 22 , 1 << 23 , 1 << 23 , 1 << 24 , 1 << 25 , 1 << 26 ,] ; const PRESET_TO_DEPTH_LIMIT : & 'static [i32] = & [4 , 8 , 24 , 48] ; # [doc = " Creates new LZMA encoding options with specified parameters."] # [allow (clippy :: too_many_arguments)] pub fn new (dict_size : u32 , lc : u32 , lp : u32 , pb : u32 , mode : EncodeMode , nice_len : u32 , mf : MfType , depth_limit : i32 ,) -> Self { Self { dict_size , lc , lp , pb , mode , nice_len , mf , depth_limit , preset_dict : None , } } # [doc = " preset: [0..9]"] # [inline] pub fn with_preset (preset : u32) -> Self { let mut opt = Self { dict_size : Default :: default () , lc : Default :: default () , lp : Default :: default () , pb : Default :: default () , mode : EncodeMode :: Normal , nice_len : Default :: default () , mf : Default :: default () , depth_limit : Default :: default () , preset_dict : Default :: default () , } ; opt . set_preset (preset) ; opt } # [doc = " preset: [0..9]"] pub fn set_preset (& mut self , preset : u32) { let preset = preset . min (9) ; self . lc = Self :: LC_DEFAULT ; self . lp = Self :: LP_DEFAULT ; self . pb = Self :: PB_DEFAULT ; self . dict_size = Self :: PRESET_TO_DICT_SIZE [preset as usize] ; if preset <= 3 { self . mode = EncodeMode :: Fast ; self . mf = MfType :: Hc4 ; self . nice_len = if preset <= 1 { 128 } else { Self :: NICE_LEN_MAX } ; self . depth_limit = Self :: PRESET_TO_DEPTH_LIMIT [preset as usize] ; } else { self . mode = EncodeMode :: Normal ; self . mf = MfType :: Bt4 ; self . nice_len = if preset == 4 { 16 } else if preset == 5 { 32 } else { 64 } ; self . depth_limit = 0 ; } } # [doc = " Returns the estimated memory usage in kilobytes for these options."] pub fn get_memory_usage (& self) -> u32 { let dict_size = self . dict_size ; let extra_size_before = get_extra_size_before (dict_size) ; 70 + LzmaEncoder :: get_mem_usage (self . mode , dict_size , extra_size_before , self . mf) } # [doc = " Returns the LZMA properties byte for these options."] # [inline (always)] pub fn get_props (& self) -> u8 { ((self . pb * 5 + self . lp) * 9 + self . lc) as u8 } }
    };
}

impl_207!();