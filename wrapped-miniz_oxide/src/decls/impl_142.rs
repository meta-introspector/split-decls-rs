macro_rules! deps {
    () => {
        BlockBoundaryState!();
        State!();
        DecompressorOxide!();
        BitBuffer!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl DecompressorOxide { # [doc = " Create a new tinfl_decompressor with all fields set to 0."] pub fn new () -> DecompressorOxide { DecompressorOxide :: default () } # [doc = " Set the current state to `Start`."] # [inline] pub fn init (& mut self) { self . state = core :: State :: Start ; } # [doc = " Returns the adler32 checksum of the currently decompressed data."] # [doc = " Note: Will return Some(1) if decompressing zlib but ignoring adler32."] # [inline] # [cfg (not (feature = "rustc-dep-of-std"))] pub fn adler32 (& self) -> Option < u32 > { if self . state != State :: Start && ! self . state . is_failure () && self . z_header0 != 0 { Some (self . check_adler32) } else { None } } # [doc = " Returns the adler32 that was read from the zlib header if it exists."] # [inline] # [cfg (not (feature = "rustc-dep-of-std"))] pub fn adler32_header (& self) -> Option < u32 > { if self . state != State :: Start && self . state != State :: BadZlibHeader && self . z_header0 != 0 { Some (self . z_adler32) } else { None } } # [cfg (all (test , feature = "with-alloc"))] pub (crate) const fn zlib_header (& self) -> (u32 , u32) { (self . z_header0 , self . z_header1) } # [doc = " Returns the current [`BlockBoundaryState`]. Should only be called when"] # [doc = " [`decompress()`] has returned [`TINFLStatus::BlockBoundary`];"] # [doc = " otherwise this will return `None`."] # [cfg (feature = "block-boundary")] pub fn block_boundary_state (& self) -> Option < BlockBoundaryState > { if self . state == core :: State :: ReadBlockHeader { assert ! (self . num_bits < 8) ; Some (BlockBoundaryState { num_bits : self . num_bits as u8 , bit_buf : self . bit_buf as u8 , z_header0 : self . z_header0 , z_header1 : self . z_header1 , check_adler32 : self . check_adler32 , }) } else { None } } # [doc = " Creates a new `DecompressorOxide` from the state returned by"] # [doc = " `block_boundary_state()`."] # [doc = ""] # [doc = " When calling [`decompress()`], the 32KiB of `out` preceding `out_pos` must be"] # [doc = " initialized with the same data that it contained when `block_boundary_state()`"] # [doc = " was called."] # [cfg (feature = "block-boundary")] pub fn from_block_boundary_state (st : & BlockBoundaryState) -> Self { DecompressorOxide { state : core :: State :: ReadBlockHeader , num_bits : st . num_bits as u32 , bit_buf : st . bit_buf as BitBuffer , z_header0 : st . z_header0 , z_header1 : st . z_header1 , z_adler32 : 1 , check_adler32 : st . check_adler32 , .. DecompressorOxide :: default () } } }
    };
}

impl_142!();