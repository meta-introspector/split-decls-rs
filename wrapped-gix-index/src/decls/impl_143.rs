macro_rules! deps {
    () => {
        State!();
        Entry!();
        Error!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl State { # [doc = " Assure our entries are consistent."] pub fn verify_entries (& self) -> Result < () , entries :: Error > { let _span = gix_features :: trace :: coarse ! ("gix_index::File::verify_entries()") ; let mut previous = None :: < & crate :: Entry > ; for (idx , entry) in self . entries . iter () . enumerate () { if let Some (prev) = previous { if prev . cmp (entry , self) != Ordering :: Less { return Err (entries :: Error :: OutOfOrder { current_index : idx , current_path : entry . path (self) . into () , current_stage : entry . flags . stage () as u8 , previous_path : prev . path (self) . into () , previous_stage : prev . flags . stage () as u8 , }) ; } } previous = Some (entry) ; } Ok (()) } # [doc = " Note: `objects` cannot be `Option<F>` as we can't call it with a closure then due to the indirection through `Some`."] pub fn verify_extensions (& self , use_find : bool , objects : impl gix_object :: Find) -> Result < () , extensions :: Error > { self . tree () . map (| t | t . verify (use_find , objects)) . transpose () ? ; Ok (()) } }
    };
}

impl_143!()