macro_rules! deps {
    () => {
        Error!();
        LinkeditDataCommand!();
        ReadRef!();
        ExportsTrieIterator!();
        Result!();
        Endian!();
        FunctionStartsIterator!();
    };
}

macro_rules! impl_562 {
    () => {
        deps!();
        impl < E : Endian > macho :: LinkeditDataCommand < E > { # [doc = " Return an iterator over the function start addresses."] # [doc = ""] # [doc = " Only works if the command is a `LC_FUNCTION_STARTS` command."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `text_segment_addr` - The VM address of the __TEXT segment."] pub fn function_starts < 'data , R : ReadRef < 'data > > (& self , endian : E , data : R , text_segment_addr : u64 ,) -> Result < FunctionStartsIterator < 'data > > { if self . cmd . get (endian) != macho :: LC_FUNCTION_STARTS { return Err (Error ("Not a function starts command")) ; } let data = data . read_bytes_at (self . dataoff . get (endian) . into () , self . datasize . get (endian) . into () ,) . read_error ("Invalid function starts offset or size") ? ; Ok (FunctionStartsIterator :: new (data , text_segment_addr)) } # [doc = " Return an iterator over the exports trie."] # [doc = ""] # [doc = " Only works if the command is a `LC_DYLD_EXPORTS_TRIE` command."] pub fn exports_trie < 'data , R : ReadRef < 'data > > (& self , endian : E , data : R ,) -> Result < ExportsTrieIterator < 'data > > { if self . cmd . get (endian) != macho :: LC_DYLD_EXPORTS_TRIE { return Err (Error ("Not an exports trie command")) ; } let data = data . read_bytes_at (self . dataoff . get (endian) . into () , self . datasize . get (endian) . into () ,) . read_error ("Invalid exports trie offset or size") ? ; Ok (ExportsTrieIterator :: new (data)) } }
    };
}

impl_562!();