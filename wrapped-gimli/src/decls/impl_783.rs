macro_rules! deps {
    () => {
        RangeListTable!();
        Writer!();
        Range!();
        Result!();
        Encoding!();
        RangeList!();
        DebugRanges!();
        Sections!();
        Error!();
        DebugRngLists!();
        Address!();
    };
}

macro_rules! impl_783 {
    () => {
        deps!();
        impl RangeListTable { # [doc = " Add a range list to the table."] pub fn add (& mut self , range_list : RangeList) -> RangeListId { let (index , _) = self . ranges . insert_full (range_list) ; RangeListId :: new (self . base_id , index) } # [doc = " Get a reference to a location list."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `id` is invalid."] # [inline] pub fn get (& self , id : RangeListId) -> & RangeList { debug_assert_eq ! (self . base_id , id . base_id) ; & self . ranges [id . index] } # [doc = " Write the range list table to the appropriate section for the given DWARF version."] pub (crate) fn write < W : Writer > (& self , sections : & mut Sections < W > , encoding : Encoding ,) -> Result < RangeListOffsets > { if self . ranges . is_empty () { return Ok (RangeListOffsets :: none ()) ; } match encoding . version { 2 ..= 4 => self . write_ranges (& mut sections . debug_ranges , encoding . address_size) , 5 => self . write_rnglists (& mut sections . debug_rnglists , encoding) , _ => Err (Error :: UnsupportedVersion (encoding . version)) , } } # [doc = " Write the range list table to the `.debug_ranges` section."] fn write_ranges < W : Writer > (& self , w : & mut DebugRanges < W > , address_size : u8 ,) -> Result < RangeListOffsets > { let mut offsets = Vec :: new () ; for range_list in self . ranges . iter () { offsets . push (w . offset ()) ; for range in & range_list . 0 { match * range { Range :: BaseAddress { address } => { let marker = ! 0 >> (64 - address_size * 8) ; w . write_udata (marker , address_size) ? ; w . write_address (address , address_size) ? ; } Range :: OffsetPair { begin , end } => { if begin == end { return Err (Error :: InvalidRange) ; } w . write_udata (begin , address_size) ? ; w . write_udata (end , address_size) ? ; } Range :: StartEnd { begin , end } => { if begin == end { return Err (Error :: InvalidRange) ; } w . write_address (begin , address_size) ? ; w . write_address (end , address_size) ? ; } Range :: StartLength { begin , length } => { let end = match begin { Address :: Constant (begin) => Address :: Constant (begin + length) , Address :: Symbol { symbol , addend } => Address :: Symbol { symbol , addend : addend + length as i64 , } , } ; if begin == end { return Err (Error :: InvalidRange) ; } w . write_address (begin , address_size) ? ; w . write_address (end , address_size) ? ; } } } w . write_udata (0 , address_size) ? ; w . write_udata (0 , address_size) ? ; } Ok (RangeListOffsets { base_id : self . base_id , offsets , }) } # [doc = " Write the range list table to the `.debug_rnglists` section."] fn write_rnglists < W : Writer > (& self , w : & mut DebugRngLists < W > , encoding : Encoding ,) -> Result < RangeListOffsets > { let mut offsets = Vec :: new () ; if encoding . version != 5 { return Err (Error :: NeedVersion (5)) ; } let length_offset = w . write_initial_length (encoding . format) ? ; let length_base = w . len () ; w . write_u16 (encoding . version) ? ; w . write_u8 (encoding . address_size) ? ; w . write_u8 (0) ? ; w . write_u32 (0) ? ; for range_list in self . ranges . iter () { offsets . push (w . offset ()) ; for range in & range_list . 0 { match * range { Range :: BaseAddress { address } => { w . write_u8 (crate :: constants :: DW_RLE_base_address . 0) ? ; w . write_address (address , encoding . address_size) ? ; } Range :: OffsetPair { begin , end } => { w . write_u8 (crate :: constants :: DW_RLE_offset_pair . 0) ? ; w . write_uleb128 (begin) ? ; w . write_uleb128 (end) ? ; } Range :: StartEnd { begin , end } => { w . write_u8 (crate :: constants :: DW_RLE_start_end . 0) ? ; w . write_address (begin , encoding . address_size) ? ; w . write_address (end , encoding . address_size) ? ; } Range :: StartLength { begin , length } => { w . write_u8 (crate :: constants :: DW_RLE_start_length . 0) ? ; w . write_address (begin , encoding . address_size) ? ; w . write_uleb128 (length) ? ; } } } w . write_u8 (crate :: constants :: DW_RLE_end_of_list . 0) ? ; } let length = (w . len () - length_base) as u64 ; w . write_initial_length_at (length_offset , length , encoding . format) ? ; Ok (RangeListOffsets { base_id : self . base_id , offsets , }) } }
    };
}

impl_783!();