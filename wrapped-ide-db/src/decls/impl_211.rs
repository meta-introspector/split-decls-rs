macro_rules! deps {
    () => {
        SymbolIndex!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl SymbolIndex { fn new (mut symbols : Box < [FileSymbol] >) -> SymbolIndex { fn cmp (lhs : & FileSymbol , rhs : & FileSymbol) -> Ordering { let lhs_chars = lhs . name . as_str () . chars () . map (| c | c . to_ascii_lowercase ()) ; let rhs_chars = rhs . name . as_str () . chars () . map (| c | c . to_ascii_lowercase ()) ; lhs_chars . cmp (rhs_chars) } symbols . par_sort_by (cmp) ; let mut builder = fst :: MapBuilder :: memory () ; let mut last_batch_start = 0 ; for idx in 0 .. symbols . len () { if let Some (next_symbol) = symbols . get (idx + 1) && cmp (& symbols [last_batch_start] , next_symbol) == Ordering :: Equal { continue ; } let start = last_batch_start ; let end = idx + 1 ; last_batch_start = end ; let key = symbols [start] . name . as_str () . to_ascii_lowercase () ; let value = SymbolIndex :: range_to_map_value (start , end) ; builder . insert (key , value) . unwrap () ; } let map = builder . into_inner () . and_then (| mut buf | { fst :: Map :: new ({ buf . shrink_to_fit () ; buf }) }) . unwrap () ; SymbolIndex { symbols , map } } pub fn len (& self) -> usize { self . symbols . len () } pub fn memory_size (& self) -> usize { self . map . as_fst () . size () + self . symbols . len () * size_of :: < FileSymbol > () } fn range_to_map_value (start : usize , end : usize) -> u64 { debug_assert ! [start <= (u32 :: MAX as usize)] ; debug_assert ! [end <= (u32 :: MAX as usize)] ; ((start as u64) << 32) | end as u64 } fn map_value_to_range (value : u64) -> (usize , usize) { let end = value as u32 as usize ; let start = (value >> 32) as usize ; (start , end) } }
    };
}

impl_211!()