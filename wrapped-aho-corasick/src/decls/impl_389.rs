macro_rules! deps {
    () => {
        RareBytesTwo!();
        RareByteOffsets!();
        ByteSet!();
        Prefilter!();
        RareBytesOne!();
        RareBytesThree!();
        RareByteOffset!();
        PrefilterI!();
        RareBytesBuilder!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl RareBytesBuilder { # [doc = " Create a new builder for constructing a rare byte prefilter."] fn new () -> RareBytesBuilder { RareBytesBuilder { ascii_case_insensitive : false , rare_set : ByteSet :: empty () , byte_offsets : RareByteOffsets :: empty () , available : true , count : 0 , rank_sum : 0 , } } # [doc = " Enable ASCII case insensitivity. When set, byte strings added to this"] # [doc = " builder will be interpreted without respect to ASCII case."] fn ascii_case_insensitive (mut self , yes : bool) -> RareBytesBuilder { self . ascii_case_insensitive = yes ; self } # [doc = " Build the rare bytes prefilter."] # [doc = ""] # [doc = " If there are more than 3 distinct rare bytes found, or if heuristics"] # [doc = " otherwise determine that this prefilter should not be used, then `None`"] # [doc = " is returned."] fn build (& self) -> Option < Prefilter > { # [cfg (feature = "perf-literal")] fn imp (builder : & RareBytesBuilder) -> Option < Prefilter > { if ! builder . available || builder . count > 3 { return None ; } let (mut bytes , mut len) = ([0 ; 3] , 0) ; for b in 0 ..= 255 { if builder . rare_set . contains (b) { bytes [len] = b ; len += 1 ; } } let finder : Arc < dyn PrefilterI > = match len { 0 => return None , 1 => Arc :: new (RareBytesOne { byte1 : bytes [0] , offset : builder . byte_offsets . set [bytes [0] as usize] , }) , 2 => Arc :: new (RareBytesTwo { offsets : builder . byte_offsets , byte1 : bytes [0] , byte2 : bytes [1] , }) , 3 => Arc :: new (RareBytesThree { offsets : builder . byte_offsets , byte1 : bytes [0] , byte2 : bytes [1] , byte3 : bytes [2] , }) , _ => unreachable ! () , } ; Some (Prefilter { finder , memory_usage : 0 }) } # [cfg (not (feature = "perf-literal"))] fn imp (_ : & RareBytesBuilder) -> Option < Prefilter > { None } imp (self) } # [doc = " Add a byte string to this builder."] # [doc = ""] # [doc = " All patterns added to an Aho-Corasick automaton should be added to this"] # [doc = " builder before attempting to construct the prefilter."] fn add (& mut self , bytes : & [u8]) { if ! self . available { return ; } if self . count > 3 { self . available = false ; return ; } if bytes . len () >= 256 { self . available = false ; return ; } let mut rarest = match bytes . first () { None => return , Some (& b) => (b , freq_rank (b)) , } ; let mut found = false ; for (pos , & b) in bytes . iter () . enumerate () { self . set_offset (pos , b) ; if found { continue ; } if self . rare_set . contains (b) { found = true ; continue ; } let rank = freq_rank (b) ; if rank < rarest . 1 { rarest = (b , rank) ; } } if ! found { self . add_rare_byte (rarest . 0) ; } } fn set_offset (& mut self , pos : usize , byte : u8) { let offset = RareByteOffset :: new (pos) . unwrap () ; self . byte_offsets . set (byte , offset) ; if self . ascii_case_insensitive { self . byte_offsets . set (opposite_ascii_case (byte) , offset) ; } } fn add_rare_byte (& mut self , byte : u8) { self . add_one_rare_byte (byte) ; if self . ascii_case_insensitive { self . add_one_rare_byte (opposite_ascii_case (byte)) ; } } fn add_one_rare_byte (& mut self , byte : u8) { if ! self . rare_set . contains (byte) { self . rare_set . add (byte) ; self . count += 1 ; self . rank_sum += freq_rank (byte) as u16 ; } } }
    };
}

impl_389!()