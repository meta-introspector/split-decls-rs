macro_rules! deps {
    () => {
        Prefilter!();
        StartBytesOne!();
        StartBytesBuilder!();
        PrefilterI!();
        StartBytesThree!();
        StartBytesTwo!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl StartBytesBuilder { # [doc = " Create a new builder for constructing a start byte prefilter."] fn new () -> StartBytesBuilder { StartBytesBuilder { ascii_case_insensitive : false , byteset : vec ! [false ; 256] , count : 0 , rank_sum : 0 , } } # [doc = " Enable ASCII case insensitivity. When set, byte strings added to this"] # [doc = " builder will be interpreted without respect to ASCII case."] fn ascii_case_insensitive (mut self , yes : bool) -> StartBytesBuilder { self . ascii_case_insensitive = yes ; self } # [doc = " Build the starting bytes prefilter."] # [doc = ""] # [doc = " If there are more than 3 distinct starting bytes, or if heuristics"] # [doc = " otherwise determine that this prefilter should not be used, then `None`"] # [doc = " is returned."] fn build (& self) -> Option < Prefilter > { # [cfg (feature = "perf-literal")] fn imp (builder : & StartBytesBuilder) -> Option < Prefilter > { if builder . count > 3 { return None ; } let (mut bytes , mut len) = ([0 ; 3] , 0) ; for b in 0 .. 256 { if ! builder . byteset [b] { continue ; } if b > 0x7F { return None ; } bytes [len] = b as u8 ; len += 1 ; } let finder : Arc < dyn PrefilterI > = match len { 0 => return None , 1 => Arc :: new (StartBytesOne { byte1 : bytes [0] }) , 2 => Arc :: new (StartBytesTwo { byte1 : bytes [0] , byte2 : bytes [1] , }) , 3 => Arc :: new (StartBytesThree { byte1 : bytes [0] , byte2 : bytes [1] , byte3 : bytes [2] , }) , _ => unreachable ! () , } ; Some (Prefilter { finder , memory_usage : 0 }) } # [cfg (not (feature = "perf-literal"))] fn imp (_ : & StartBytesBuilder) -> Option < Prefilter > { None } imp (self) } # [doc = " Add a byte string to this builder."] # [doc = ""] # [doc = " All patterns added to an Aho-Corasick automaton should be added to this"] # [doc = " builder before attempting to construct the prefilter."] fn add (& mut self , bytes : & [u8]) { if self . count > 3 { return ; } if let Some (& byte) = bytes . first () { self . add_one_byte (byte) ; if self . ascii_case_insensitive { self . add_one_byte (opposite_ascii_case (byte)) ; } } } fn add_one_byte (& mut self , byte : u8) { if ! self . byteset [byte as usize] { self . byteset [byte as usize] = true ; self . count += 1 ; self . rank_sum += freq_rank (byte) as u16 ; } } }
    };
}

impl_397!();