macro_rules! deps {
    () => {
        Result!();
        LzmaWriter!();
        Write!();
        RangeEncoder!();
        LzmaEncoder!();
        AutoFinisher!();
        LzmaOptions!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl < W : Write > LzmaWriter < W > { # [doc = " Creates a new LZMA writer with full control over formatting options."] pub fn new (mut out : W , options : & LzmaOptions , use_header : bool , use_end_marker : bool , expected_uncompressed_size : Option < u64 > ,) -> crate :: Result < LzmaWriter < W > > { let (mut lzma , mode) = LzmaEncoder :: new (options . mode , options . lc , options . lp , options . pb , options . mf , options . depth_limit , options . dict_size , options . nice_len as usize ,) ; if let Some (preset_dict) = & options . preset_dict { if use_header { return Err (error_unsupported ("header is not supported with preset dict" ,)) ; } lzma . lz . set_preset_dict (options . dict_size , preset_dict) ; } let props = options . get_props () ; if use_header { out . write_all (& [props]) ? ; let dict_size = options . dict_size ; out . write_all (& dict_size . to_le_bytes ()) ? ; let expected_compressed_size = expected_uncompressed_size . unwrap_or (u64 :: MAX) ; out . write_all (& expected_compressed_size . to_le_bytes ()) ? ; } let rc = RangeEncoder :: new (out) ; Ok (LzmaWriter { rc , lzma , use_end_marker , current_uncompressed_size : 0 , expected_uncompressed_size , props , mode , }) } # [doc = " Creates a new LZMA writer that includes a .lzma header with the specified input size."] # [inline] pub fn new_use_header (out : W , options : & LzmaOptions , input_size : Option < u64 > ,) -> crate :: Result < Self > { Self :: new (out , options , true , input_size . is_none () , input_size) } # [doc = " Creates a new LZMA writer without a .lzma header."] # [inline] pub fn new_no_header (out : W , options : & LzmaOptions , use_end_marker : bool ,) -> crate :: Result < Self > { Self :: new (out , options , false , use_end_marker , None) } # [doc = " Returns a wrapper around `self` that will finish the stream on drop."] pub fn auto_finish (self) -> AutoFinisher < Self > { AutoFinisher (Some (self)) } # [doc = " Returns the LZMA properties byte."] # [inline] pub fn props (& self) -> u8 { self . props } # [doc = " Returns the number of uncompressed bytes written so far."] # [inline] pub fn get_uncompressed_size (& self) -> u64 { self . current_uncompressed_size } # [doc = " Unwraps the writer, returning the underlying writer."] pub fn into_inner (self) -> W { self . rc . into_inner () } # [doc = " Returns a reference to the inner writer."] pub fn inner (& self) -> & W { self . rc . inner () } # [doc = " Returns a mutable reference to the inner writer."] pub fn inner_mut (& mut self) -> & mut W { self . rc . inner_mut () } # [doc = " Finishes the compression and returns the underlying writer."] pub fn finish (mut self) -> crate :: Result < W > { if let Some (exp) = self . expected_uncompressed_size { if exp != self . current_uncompressed_size { return Err (error_invalid_input ("expected compressed size does not match actual compressed size" ,)) ; } } self . lzma . lz . set_finishing () ; self . lzma . encode_for_lzma1 (& mut self . rc , & mut self . mode) ? ; if self . use_end_marker { self . lzma . encode_lzma1_end_marker (& mut self . rc) ? ; } self . rc . finish () ? ; let Self { rc , .. } = self ; Ok (rc . into_inner ()) } }
    };
}

impl_225!();