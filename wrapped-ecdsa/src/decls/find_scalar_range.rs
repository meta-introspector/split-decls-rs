macro_rules! find_scalar_range {
    () => {
        # [doc = " Locate the range within a slice at which a particular subslice is located"] fn find_scalar_range (outer : & [u8] , inner : & [u8]) -> Result < Range < usize > > { let outer_start = outer . as_ptr () as usize ; let inner_start = inner . as_ptr () as usize ; let start = inner_start . checked_sub (outer_start) . ok_or_else (Error :: new) ? ; let end = start . checked_add (inner . len ()) . ok_or_else (Error :: new) ? ; Ok (Range { start , end }) }
    };
}

find_scalar_range!();