macro_rules! deps {
    () => {
        DebugAddrBase!();
        Reader!();
        DebugAddrOffset!();
        AddrHeaderIter!();
        Result!();
        DebugAddr!();
        DebugAddrIndex!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < R : Reader > DebugAddr < R > { # [doc = " Returns the address at the given `base` and `index`."] # [doc = ""] # [doc = " A set of addresses in the `.debug_addr` section consists of a header"] # [doc = " followed by a series of addresses."] # [doc = ""] # [doc = " The `base` must be the `DW_AT_addr_base` value from the compilation unit DIE."] # [doc = " This is an offset that points to the first address following the header."] # [doc = ""] # [doc = " The `index` is the value of a `DW_FORM_addrx` attribute."] # [doc = ""] # [doc = " The `address_size` must be the size of the address for the compilation unit."] # [doc = " This value must also match the header. However, note that we do not parse the"] # [doc = " header to validate this, since locating the header is unreliable, and the GNU"] # [doc = " extensions do not emit it."] pub fn get_address (& self , address_size : u8 , base : DebugAddrBase < R :: Offset > , index : DebugAddrIndex < R :: Offset > ,) -> Result < u64 > { let input = & mut self . section . clone () ; input . skip (base . 0) ? ; input . skip (R :: Offset :: from_u64 (index . 0 . into_u64 () * u64 :: from (address_size) ,) ?) ? ; input . read_address (address_size) } # [doc = " Iterate the sets of entries in the `.debug_addr` section."] # [doc = ""] # [doc = " Each set of entries belongs to a single unit."] pub fn headers (& self) -> AddrHeaderIter < R > { AddrHeaderIter { input : self . section . clone () , offset : DebugAddrOffset (R :: Offset :: from_u8 (0)) , } } }
    };
}

impl_144!()