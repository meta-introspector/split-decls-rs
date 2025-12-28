macro_rules! deps {
    () => {
        LookupEntryIter!();
        LookupParser!();
        Reader!();
        Result!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl < R , Parser > LookupEntryIter < R , Parser > where R : Reader , Parser : LookupParser < R > , { # [doc = " Advance the iterator and return the next entry."] # [doc = ""] # [doc = " Returns the newly parsed entry as `Ok(Some(Parser::Entry))`. Returns"] # [doc = " `Ok(None)` when iteration is complete and all entries have already been"] # [doc = " parsed and yielded. If an error occurs while parsing the next entry,"] # [doc = " then this error is returned as `Err(e)`, and all subsequent calls return"] # [doc = " `Ok(None)`."] # [doc = ""] # [doc = " Can be [used with `FallibleIterator`](./index.html#using-with-fallibleiterator)."] pub fn next (& mut self) -> Result < Option < Parser :: Entry > > { loop { if let Some ((ref mut input , ref header)) = self . current_set { if ! input . is_empty () { match Parser :: parse_entry (input , header) { Ok (Some (entry)) => return Ok (Some (entry)) , Ok (None) => { } Err (e) => { input . empty () ; self . remaining_input . empty () ; return Err (e) ; } } } } if self . remaining_input . is_empty () { self . current_set = None ; return Ok (None) ; } match Parser :: parse_header (& mut self . remaining_input) { Ok (set) => { self . current_set = Some (set) ; } Err (e) => { self . current_set = None ; self . remaining_input . empty () ; return Err (e) ; } } } } }
    };
}

impl_475!();