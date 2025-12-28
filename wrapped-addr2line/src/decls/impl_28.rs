macro_rules! deps {
    () => {
        UnitRef!();
        LazyFunction!();
        DebugFile!();
        Result!();
        Functions!();
        Error!();
        Context!();
        RangeAttributes!();
        FunctionAddress!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < R : gimli :: Reader > Functions < R > { fn parse (unit : gimli :: UnitRef < R >) -> Result < Functions < R > , Error > { let mut functions = Vec :: new () ; let mut addresses = Vec :: new () ; let mut entries = unit . entries_raw (None) ? ; while ! entries . is_empty () { let dw_die_offset = entries . next_offset () ; if let Some (abbrev) = entries . read_abbreviation () ? { if abbrev . tag () == gimli :: DW_TAG_subprogram { let mut ranges = RangeAttributes :: default () ; for spec in abbrev . attributes () { match entries . read_attribute (* spec) { Ok (ref attr) => { match attr . name () { gimli :: DW_AT_low_pc => match attr . value () { gimli :: AttributeValue :: Addr (val) => { ranges . low_pc = Some (val) } gimli :: AttributeValue :: DebugAddrIndex (index) => { ranges . low_pc = Some (unit . address (index) ?) ; } _ => { } } , gimli :: DW_AT_high_pc => match attr . value () { gimli :: AttributeValue :: Addr (val) => { ranges . high_pc = Some (val) } gimli :: AttributeValue :: DebugAddrIndex (index) => { ranges . high_pc = Some (unit . address (index) ?) ; } gimli :: AttributeValue :: Udata (val) => { ranges . size = Some (val) } _ => { } } , gimli :: DW_AT_ranges => { ranges . ranges_offset = unit . attr_ranges_offset (attr . value ()) ? ; } _ => { } } ; } Err (e) => return Err (e) , } } let function_index = functions . len () ; let has_address = ranges . for_each_range (unit , | range | { addresses . push (FunctionAddress { range , function : function_index , }) ; }) ? ; if has_address { functions . push (LazyFunction :: new (dw_die_offset)) ; } } else { entries . skip_attributes (abbrev . attributes ()) ? ; } } } addresses . sort_unstable_by_key (| x | x . range . begin) ; Ok (Functions { functions : functions . into_boxed_slice () , addresses : addresses . into_boxed_slice () , }) } pub (crate) fn find_address (& self , probe : u64) -> Option < usize > { self . addresses . binary_search_by (| address | { if probe < address . range . begin { Ordering :: Greater } else if probe >= address . range . end { Ordering :: Less } else { Ordering :: Equal } }) . ok () } pub (crate) fn parse_inlined_functions (& self , file : DebugFile , unit : gimli :: UnitRef < R > , ctx : & Context < R > ,) -> Result < () , Error > { for function in & * self . functions { function . borrow (file , unit , ctx) ? ; } Ok (()) } }
    };
}

impl_28!()