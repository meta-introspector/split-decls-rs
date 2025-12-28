macro_rules! deps {
    () => {
        Reader!();
        LocListsFormat!();
        Result!();
        DebugAddrIndex!();
        Error!();
        RawLocListEntry!();
        RawRange!();
        Expression!();
        Encoding!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl < R : Reader > RawLocListEntry < R > { # [doc = " Parse a location list entry from `.debug_loclists`"] fn parse (input : & mut R , encoding : Encoding , format : LocListsFormat) -> Result < Option < Self > > { Ok (match format { LocListsFormat :: Bare => { let range = RawRange :: parse (input , encoding . address_size) ? ; if range . is_end () { None } else if range . is_base_address (encoding . address_size) { Some (RawLocListEntry :: BaseAddress { addr : range . end }) } else { let len = R :: Offset :: from_u16 (input . read_u16 () ?) ; let data = Expression (input . split (len) ?) ; Some (RawLocListEntry :: AddressOrOffsetPair { begin : range . begin , end : range . end , data , }) } } LocListsFormat :: Lle => match constants :: DwLle (input . read_u8 () ?) { constants :: DW_LLE_end_of_list => None , constants :: DW_LLE_base_addressx => Some (RawLocListEntry :: BaseAddressx { addr : DebugAddrIndex (input . read_uleb128 () . and_then (R :: Offset :: from_u64) ?) , }) , constants :: DW_LLE_startx_endx => Some (RawLocListEntry :: StartxEndx { begin : DebugAddrIndex (input . read_uleb128 () . and_then (R :: Offset :: from_u64) ?) , end : DebugAddrIndex (input . read_uleb128 () . and_then (R :: Offset :: from_u64) ?) , data : parse_data (input , encoding) ? , }) , constants :: DW_LLE_startx_length => Some (RawLocListEntry :: StartxLength { begin : DebugAddrIndex (input . read_uleb128 () . and_then (R :: Offset :: from_u64) ?) , length : if encoding . version >= 5 { input . read_uleb128 () ? } else { input . read_u32 () ? as u64 } , data : parse_data (input , encoding) ? , }) , constants :: DW_LLE_offset_pair => Some (RawLocListEntry :: OffsetPair { begin : input . read_uleb128 () ? , end : input . read_uleb128 () ? , data : parse_data (input , encoding) ? , }) , constants :: DW_LLE_default_location => Some (RawLocListEntry :: DefaultLocation { data : parse_data (input , encoding) ? , }) , constants :: DW_LLE_base_address => Some (RawLocListEntry :: BaseAddress { addr : input . read_address (encoding . address_size) ? , }) , constants :: DW_LLE_start_end => Some (RawLocListEntry :: StartEnd { begin : input . read_address (encoding . address_size) ? , end : input . read_address (encoding . address_size) ? , data : parse_data (input , encoding) ? , }) , constants :: DW_LLE_start_length => Some (RawLocListEntry :: StartLength { begin : input . read_address (encoding . address_size) ? , length : input . read_uleb128 () ? , data : parse_data (input , encoding) ? , }) , entry => { return Err (Error :: UnknownLocListsEntry (entry)) ; } } , }) } }
    };
}

impl_461!();