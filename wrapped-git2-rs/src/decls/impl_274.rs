macro_rules! deps {
    () => {
        Error!();
        ConfigEntries!();
        ConfigEntry!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl < 'cfg > ConfigEntries < 'cfg > { # [doc = " Advances the iterator and returns the next value."] # [doc = ""] # [doc = " Returns `None` when iteration is finished."] pub fn next (& mut self) -> Option < Result < & ConfigEntry < 'cfg > , Error > > { let mut raw = ptr :: null_mut () ; drop (self . current . take ()) ; unsafe { try_call_iter ! (raw :: git_config_next (& mut raw , self . raw)) ; let entry = ConfigEntry { owned : false , raw , _marker : marker :: PhantomData , } ; self . current = Some (entry) ; Some (Ok (self . current . as_ref () . unwrap ())) } } # [doc = " Calls the given closure for each remaining entry in the iterator."] pub fn for_each < F : FnMut (& ConfigEntry < 'cfg >) > (mut self , mut f : F) -> Result < () , Error > { while let Some (entry) = self . next () { let entry = entry ? ; f (entry) ; } Ok (()) } }
    };
}

impl_274!()