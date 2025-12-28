macro_rules! deps {
    () => {
        ArangeEntryIter!();
        ArangeEntry!();
        Result!();
        Reader!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        impl < R : Reader > ArangeEntryIter < R > { # [doc = " Advance the iterator and return the next arange."] # [doc = ""] # [doc = " Returns the newly parsed arange as `Ok(Some(arange))`. Returns `Ok(None)`"] # [doc = " when iteration is complete and all aranges have already been parsed and"] # [doc = " yielded. If an error occurs while parsing the next arange, then this error"] # [doc = " is returned as `Err(e)`, and all subsequent calls return `Ok(None)`."] pub fn next (& mut self) -> Result < Option < ArangeEntry > > { loop { let raw_entry = match self . next_raw () ? { Some (entry) => entry , None => return Ok (None) , } ; let entry = self . convert_raw (raw_entry) ? ; if entry . is_some () { return Ok (entry) ; } } } # [doc = " Advance the iterator and return the next arange without validating it."] # [doc = ""] # [doc = " The returned entry will have `range.end` set to 0."] # [doc = " This will return tombstone entries as well."] pub fn next_raw (& mut self) -> Result < Option < ArangeEntry > > { if self . input . is_empty () { return Ok (None) ; } match ArangeEntry :: parse (& mut self . input , self . encoding) { Ok (Some (entry)) => Ok (Some (entry)) , Ok (None) => { self . input . empty () ; Ok (None) } Err (e) => { self . input . empty () ; Err (e) } } } # [doc = " Convert a raw range into a range."] # [doc = ""] # [doc = " The raw range should have been obtained from `next_raw`."] # [doc (hidden)] pub fn convert_raw (& self , mut entry : ArangeEntry) -> Result < Option < ArangeEntry > > { let address_size = self . encoding . address_size ; if entry . range . begin >= u64 :: min_tombstone (address_size) { return Ok (None) ; } entry . range . end = entry . range . begin . add_sized (entry . length , address_size) ? ; Ok (Some (entry)) } }
    };
}

impl_371!();