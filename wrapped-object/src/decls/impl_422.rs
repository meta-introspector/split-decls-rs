macro_rules! deps {
    () => {
        FileHeader!();
        Version!();
        VersionTable!();
        Result!();
        ReadRef!();
        U64!();
        SymbolIndex!();
        Endian!();
        GnuHashHeader!();
        U32!();
        GnuHashTable!();
        Sym!();
        SymbolTable!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > GnuHashTable < 'data , Elf > { # [doc = " Parse a GNU hash table."] # [doc = ""] # [doc = " `data` should be from an [`elf::SHT_GNU_HASH`] section, or from a"] # [doc = " segment pointed to via the [`elf::DT_GNU_HASH`] entry."] # [doc = ""] # [doc = " The header is read at offset 0 in the given `data`."] # [doc = ""] # [doc = " The header does not contain a length field, and so all of `data`"] # [doc = " will be used as the hash table values. It does not matter if this"] # [doc = " is longer than needed, and this will often the case when accessing"] # [doc = " the hash table via the [`elf::DT_GNU_HASH`] entry."] pub fn parse (endian : Elf :: Endian , data : & 'data [u8]) -> Result < Self > { let mut offset = 0 ; let header = data . read :: < elf :: GnuHashHeader < Elf :: Endian > > (& mut offset) . read_error ("Invalid GNU hash header") ? ; let bloom_len = u64 :: from (header . bloom_count . get (endian)) * mem :: size_of :: < Elf :: Word > () as u64 ; let bloom_filters = data . read_bytes (& mut offset , bloom_len) . read_error ("Invalid GNU hash bloom filters") ? ; let buckets = data . read_slice (& mut offset , header . bucket_count . get (endian) as usize) . read_error ("Invalid GNU hash buckets") ? ; let chain_count = (data . len () - offset as usize) / 4 ; let values = data . read_slice (& mut offset , chain_count) . read_error ("Invalid GNU hash values") ? ; Ok (GnuHashTable { symbol_base : header . symbol_base . get (endian) , bloom_shift : header . bloom_shift . get (endian) , bloom_filters , buckets , values , }) } # [doc = " Return the symbol table index of the first symbol in the hash table."] pub fn symbol_base (& self) -> u32 { self . symbol_base } # [doc = " Determine the symbol table length by finding the last entry in the hash table."] # [doc = ""] # [doc = " Returns `None` if the hash table is empty or invalid."] pub fn symbol_table_length (& self , endian : Elf :: Endian) -> Option < u32 > { if self . symbol_base == 0 { return None ; } let mut max_symbol = 0 ; for bucket in self . buckets { let bucket = bucket . get (endian) ; if max_symbol < bucket { max_symbol = bucket ; } } for value in self . values . get (max_symbol . checked_sub (self . symbol_base) ? as usize ..) ? { max_symbol += 1 ; if value . get (endian) & 1 != 0 { return Some (max_symbol) ; } } None } fn bucket (& self , endian : Elf :: Endian , hash : u32) -> SymbolIndex { SymbolIndex (self . buckets [(hash as usize) % self . buckets . len ()] . get (endian) as usize) } # [doc = " Use the hash table to find the symbol table entry with the given name, hash, and version."] pub fn find < R : ReadRef < 'data > > (& self , endian : Elf :: Endian , name : & [u8] , hash : u32 , version : Option < & Version < '_ > > , symbols : & SymbolTable < 'data , Elf , R > , versions : & VersionTable < 'data , Elf > ,) -> Option < (SymbolIndex , & 'data Elf :: Sym) > { let word_bits = mem :: size_of :: < Elf :: Word > () as u32 * 8 ; let bloom_count = self . bloom_filters . len () / mem :: size_of :: < Elf :: Word > () ; let offset = ((hash / word_bits) & (bloom_count as u32 - 1)) * mem :: size_of :: < Elf :: Word > () as u32 ; let filter = if word_bits == 64 { self . bloom_filters . read_at :: < U64 < Elf :: Endian > > (offset . into ()) . ok () ? . get (endian) } else { self . bloom_filters . read_at :: < U32 < Elf :: Endian > > (offset . into ()) . ok () ? . get (endian) . into () } ; if filter & (1 << (hash % word_bits)) == 0 { return None ; } if filter & (1 << ((hash >> self . bloom_shift) % word_bits)) == 0 { return None ; } let mut index = self . bucket (endian , hash) ; if index == SymbolIndex (0) { return None ; } let strings = symbols . strings () ; let symbols = symbols . symbols () . get (index . 0 ..) ? ; let values = self . values . get (index . 0 . checked_sub (self . symbol_base as usize) ? ..) ? ; for (symbol , value) in symbols . iter () . zip (values . iter ()) { let value = value . get (endian) ; if value | 1 == hash | 1 { if symbol . name (endian , strings) == Ok (name) && versions . matches (endian , index , version) { return Some ((index , symbol)) ; } } if value & 1 != 0 { break ; } index . 0 += 1 ; } None } }
    };
}

impl_422!();