macro_rules! deps {
    () => {
        Error!();
        Location!();
        LocationRangeIter!();
        Result!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 'ctx , R : gimli :: Reader > LocationRangeIter < 'ctx , R > { fn next_loc (& mut self) -> Result < Option < (u64 , u64 , Location < 'ctx >) > , Error > { loop { let iter = self . iter . take () ; match iter { None => match self . unit_iter . next () { Some ((unit , range)) => { self . iter = unit . find_location_range (cmp :: max (self . probe_low , range . begin) , cmp :: min (self . probe_high , range . end) , self . sections ,) ? ; } None => return Ok (None) , } , Some (mut iter) => { if let item @ Some (_) = iter . next () { self . iter = Some (iter) ; return Ok (item) ; } } } } } }
    };
}

impl_89!();