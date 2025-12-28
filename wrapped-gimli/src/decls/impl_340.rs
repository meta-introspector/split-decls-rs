macro_rules! deps {
    () => {
        AbbreviationsCache!();
        DebugAbbrevOffset!();
        Result!();
        DebugInfoUnitHeadersIter!();
        DebugAbbrev!();
        Abbreviations!();
        Reader!();
        AbbreviationsCacheStrategy!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl AbbreviationsCache { # [doc = " Create an empty abbreviations cache."] pub fn new () -> Self { Self :: default () } # [doc = " Parse abbreviations and store them in the cache."] # [doc = ""] # [doc = " This will iterate over the given units to determine the abbreviations"] # [doc = " offsets. Any existing cache entries are discarded."] # [doc = ""] # [doc = " Errors during parsing abbreviations are also stored in the cache."] # [doc = " Errors during iterating over the units are ignored."] pub fn populate < R : Reader > (& mut self , strategy : AbbreviationsCacheStrategy , debug_abbrev : & DebugAbbrev < R > , mut units : DebugInfoUnitHeadersIter < R > ,) { let mut offsets = Vec :: new () ; match strategy { AbbreviationsCacheStrategy :: Duplicates => { while let Ok (Some (unit)) = units . next () { offsets . push (unit . debug_abbrev_offset ()) ; } offsets . sort_unstable_by_key (| offset | offset . 0) ; let mut prev_offset = R :: Offset :: from_u8 (0) ; let mut count = 0 ; offsets . retain (| offset | { if count == 0 || prev_offset != offset . 0 { prev_offset = offset . 0 ; count = 1 ; } else { count += 1 ; } count == 2 }) ; } AbbreviationsCacheStrategy :: All => { while let Ok (Some (unit)) = units . next () { offsets . push (unit . debug_abbrev_offset ()) ; } offsets . sort_unstable_by_key (| offset | offset . 0) ; offsets . dedup () ; } } self . abbreviations = offsets . into_iter () . map (| offset | { (offset . 0 . into_u64 () , debug_abbrev . abbreviations (offset) . map (Arc :: new) ,) }) . collect () ; } # [doc = " Set an entry in the abbreviations cache."] # [doc = ""] # [doc = " This is only required if you want to manually populate the cache."] pub fn set < R : Reader > (& mut self , offset : DebugAbbrevOffset < R :: Offset > , abbreviations : Arc < Abbreviations > ,) { self . abbreviations . insert (offset . 0 . into_u64 () , Ok (abbreviations)) ; } # [doc = " Parse the abbreviations at the given offset."] # [doc = ""] # [doc = " This uses the cache if possible, but does not update it."] pub fn get < R : Reader > (& self , debug_abbrev : & DebugAbbrev < R > , offset : DebugAbbrevOffset < R :: Offset > ,) -> Result < Arc < Abbreviations > > { match self . abbreviations . get (& offset . 0 . into_u64 ()) { Some (entry) => entry . clone () , None => debug_abbrev . abbreviations (offset) . map (Arc :: new) , } } }
    };
}

impl_340!();