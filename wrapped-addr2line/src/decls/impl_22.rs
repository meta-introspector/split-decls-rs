macro_rules! deps {
    () => {
        RangeAttributes!();
        Error!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < R : gimli :: Reader > RangeAttributes < R > { fn for_each_range < F : FnMut (gimli :: Range) > (& self , unit : gimli :: UnitRef < R > , mut f : F ,) -> Result < bool , Error > { let mut added_any = false ; let mut add_range = | range : gimli :: Range | { if range . begin < range . end { f (range) ; added_any = true } } ; if let Some (ranges_offset) = self . ranges_offset { let mut range_list = unit . ranges (ranges_offset) ? ; while let Some (range) = range_list . next () ? { add_range (range) ; } } else if let (Some (begin) , Some (end)) = (self . low_pc , self . high_pc) { add_range (gimli :: Range { begin , end }) ; } else if let (Some (begin) , Some (size)) = (self . low_pc , self . size) { let end = begin . wrapping_add (size) ; add_range (gimli :: Range { begin , end }) ; } Ok (added_any) } }
    };
}

impl_22!()