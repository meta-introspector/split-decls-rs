macro_rules! deps {
    () => {
        Reader!();
        RawRange!();
        Result!();
    };
}

macro_rules! impl_577 {
    () => {
        deps!();
        impl RawRange { # [doc = " Check if this is a range end entry."] # [inline] pub fn is_end (& self) -> bool { self . begin == 0 && self . end == 0 } # [doc = " Check if this is a base address selection entry."] # [doc = ""] # [doc = " A base address selection entry changes the base address that subsequent"] # [doc = " range entries are relative to."] # [inline] pub fn is_base_address (& self , address_size : u8) -> bool { self . begin == ! 0 >> (64 - address_size * 8) } # [doc = " Parse an address range entry from `.debug_ranges` or `.debug_loc`."] # [inline] pub fn parse < R : Reader > (input : & mut R , address_size : u8) -> Result < RawRange > { let begin = input . read_address (address_size) ? ; let end = input . read_address (address_size) ? ; let range = RawRange { begin , end } ; Ok (range) } }
    };
}

impl_577!();