macro_rules! deps {
    () => {
        PubTypesEntry!();
        Result!();
        PubTypesEntryIter!();
        Reader!();
    };
}

macro_rules! impl_548 {
    () => {
        deps!();
        impl < R : Reader > PubTypesEntryIter < R > { # [doc = " Advance the iterator and return the next pubtype."] # [doc = ""] # [doc = " Returns the newly parsed pubtype as `Ok(Some(pubtype))`. Returns"] # [doc = " `Ok(None)` when iteration is complete and all pubtypes have already been"] # [doc = " parsed and yielded. If an error occurs while parsing the next pubtype,"] # [doc = " then this error is returned as `Err(e)`, and all subsequent calls return"] # [doc = " `Ok(None)`."] pub fn next (& mut self) -> Result < Option < PubTypesEntry < R > > > { self . 0 . next () } }
    };
}

impl_548!()