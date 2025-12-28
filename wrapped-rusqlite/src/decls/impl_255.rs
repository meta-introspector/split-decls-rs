macro_rules! deps {
    () => {
        ChangesetItem!();
        Error!();
        ChangesetIter!();
        Result!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl FallibleStreamingIterator for ChangesetIter < '_ > { type Error = Error ; type Item = ChangesetItem ; # [inline] fn advance (& mut self) -> Result < () > { let rc = unsafe { ffi :: sqlite3changeset_next (self . it) } ; match rc { ffi :: SQLITE_ROW => { self . item = Some (ChangesetItem { it : self . it }) ; Ok (()) } ffi :: SQLITE_DONE => { self . item = None ; Ok (()) } code => Err (error_from_sqlite_code (code , None)) , } } # [inline] fn get (& self) -> Option < & ChangesetItem > { self . item . as_ref () } }
    };
}

impl_255!()