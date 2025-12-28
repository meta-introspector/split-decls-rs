macro_rules! deps {
    () => {
        UnitRef!();
        DwoUnit!();
        LookupContinuation!();
        DebugFile!();
        Error!();
        ResUnit!();
        SimpleLookup!();
        Functions!();
        Result!();
        Lines!();
        Location!();
        LookupResult!();
        Function!();
        LineLocationRangeIter!();
        SplitDwarfLoad!();
        Context!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < R : gimli :: Reader > ResUnit < R > { pub (crate) fn unit_ref < 'a > (& 'a self , sections : & 'a gimli :: Dwarf < R >) -> gimli :: UnitRef < 'a , R > { gimli :: UnitRef :: new (sections , & self . dw_unit) } # [doc = " Returns the DWARF sections and the unit."] # [doc = ""] # [doc = " Loads the DWO unit if necessary."] # [allow (clippy :: type_complexity)] pub (crate) fn dwarf_and_unit < 'unit , 'ctx : 'unit > (& 'unit self , ctx : & 'ctx Context < R > ,) -> LookupResult < SimpleLookup < Result < UnitRef < 'unit , R > , Error > , R , impl FnOnce (Option < Arc < gimli :: Dwarf < R > > >) -> Result < UnitRef < 'unit , R > , Error > , > , > { let map_dwo = move | dwo : & 'unit Result < Option < Box < DwoUnit < R > > > , Error > | match dwo { Ok (Some (dwo)) => Ok ((DebugFile :: Dwo , dwo . unit_ref ())) , Ok (None) => Ok ((DebugFile :: Primary , self . unit_ref (& * ctx . sections))) , Err (e) => Err (* e) , } ; let complete = | dwo | SimpleLookup :: new_complete (map_dwo (dwo)) ; if let Some (dwo) = self . dwo . get () { return complete (dwo) ; } let dwo_id = match self . dw_unit . dwo_id { None => { return complete (self . dwo . get_or_init (| | Ok (None))) ; } Some (dwo_id) => dwo_id , } ; let comp_dir = self . dw_unit . comp_dir . clone () ; let dwo_name = self . dw_unit . dwo_name () . and_then (| s | { if let Some (s) = s { Ok (Some (ctx . sections . attr_string (& self . dw_unit , s) ?)) } else { Ok (None) } }) ; let path = match dwo_name { Ok (v) => v , Err (e) => { return complete (self . dwo . get_or_init (| | Err (e))) ; } } ; let process_dwo = move | dwo_dwarf : Option < Arc < gimli :: Dwarf < R > > > | { let dwo_dwarf = match dwo_dwarf { None => return Ok (None) , Some (dwo_dwarf) => dwo_dwarf , } ; let mut dwo_units = dwo_dwarf . units () ; let dwo_header = match dwo_units . next () ? { Some (dwo_header) => dwo_header , None => return Ok (None) , } ; let mut dwo_unit = dwo_dwarf . unit (dwo_header) ? ; dwo_unit . copy_relocated_attributes (& self . dw_unit) ; Ok (Some (Box :: new (DwoUnit { sections : dwo_dwarf , dw_unit : dwo_unit , }))) } ; SimpleLookup :: new_needs_load (SplitDwarfLoad { dwo_id , comp_dir , path , parent : ctx . sections . clone () , } , move | dwo_dwarf | map_dwo (self . dwo . get_or_init (| | process_dwo (dwo_dwarf))) ,) } pub (crate) fn parse_lines (& self , sections : & gimli :: Dwarf < R >) -> Result < Option < & Lines > , Error > { let ilnp = match self . dw_unit . line_program { Some (ref ilnp) => ilnp , None => return Ok (None) , } ; self . lines . borrow (self . unit_ref (sections) , ilnp) . map (Some) } pub (crate) fn parse_functions < 'unit , 'ctx : 'unit > (& 'unit self , ctx : & 'ctx Context < R > ,) -> LookupResult < impl LookupContinuation < Output = Result < & 'unit Functions < R > , Error > , Buf = R > > { self . dwarf_and_unit (ctx) . map (move | r | { let (_file , unit) = r ? ; self . functions . borrow (unit) }) } pub (crate) fn parse_inlined_functions < 'unit , 'ctx : 'unit > (& 'unit self , ctx : & 'ctx Context < R > ,) -> LookupResult < impl LookupContinuation < Output = Result < () , Error > , Buf = R > + 'unit > { self . dwarf_and_unit (ctx) . map (move | r | { let (file , unit) = r ? ; self . functions . borrow (unit) ? . parse_inlined_functions (file , unit , ctx) }) } pub (crate) fn find_location (& self , probe : u64 , sections : & gimli :: Dwarf < R > ,) -> Result < Option < Location < '_ > > , Error > { let Some (lines) = self . parse_lines (sections) ? else { return Ok (None) ; } ; lines . find_location (probe) } # [inline] pub (crate) fn find_location_range (& self , probe_low : u64 , probe_high : u64 , sections : & gimli :: Dwarf < R > ,) -> Result < Option < LineLocationRangeIter < '_ > > , Error > { let Some (lines) = self . parse_lines (sections) ? else { return Ok (None) ; } ; lines . find_location_range (probe_low , probe_high) . map (Some) } # [allow (clippy :: type_complexity)] pub (crate) fn find_function_or_location < 'unit , 'ctx : 'unit > (& 'unit self , probe : u64 , ctx : & 'ctx Context < R > ,) -> LookupResult < impl LookupContinuation < Output = Result < (Option < & 'unit Function < R > > , Option < Location < 'unit > >) , Error > , Buf = R , > , > { self . dwarf_and_unit (ctx) . map (move | r | { let (file , unit) = r ? ; let functions = self . functions . borrow (unit) ? ; let function = match functions . find_address (probe) { Some (address) => { let function_index = functions . addresses [address] . function ; let function = & functions . functions [function_index] ; Some (function . borrow (file , unit , ctx) ?) } None => None , } ; let location = self . find_location (probe , & ctx . sections) ? ; Ok ((function , location)) }) } }
    };
}

impl_79!();