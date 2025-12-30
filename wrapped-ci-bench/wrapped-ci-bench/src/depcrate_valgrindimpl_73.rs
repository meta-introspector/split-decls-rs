// Generated macro for impl_73 (impl)
macro_rules! Depcrate_valgrindimpl_73 {
() => {
// Module: crate::valgrind
// Provides: {"impl_73"}
// Dependencies: {}
impl MemoryDetails { # [doc = " Returns the heap usage, extracted from the DHAT log file at the provided path"] fn from_file (file : & Path) -> anyhow :: Result < Self > { let file_in = File :: open (file) . context ("Unable to open DHAT log file") ? ; let mut out = Self :: default () ; for line in BufReader :: new (file_in) . lines () { let line = line . context ("Error reading DHAT log file") ? ; match line . split_whitespace () . collect :: < Vec < & str > > () . as_slice () { [_ , "Total:" , bytes , "bytes" , "in" , blocks , "blocks"] => { out . heap_total_bytes = parse_u64 (bytes) ; out . heap_total_blocks = parse_u64 (blocks) ; } [_ , "At" , "t-gmax:" , bytes , "bytes" , "in" , blocks , "blocks"] => { out . heap_peak_bytes = parse_u64 (bytes) ; out . heap_peak_blocks = parse_u64 (blocks) ; } _ => { } } } fn parse_u64 (s : & str) -> u64 { s . replace ("," , "") . parse () . unwrap () } Ok (out) } }
};
}
