// Generated macro for parse_cachegrind_output (function)
macro_rules! Depcrateparse_cachegrind_output {
() => {
// Module: crate
// Provides: {"parse_cachegrind_output"}
// Dependencies: {}
fn parse_cachegrind_output (file : & Path) -> CachegrindStats { let mut events_line = None ; let mut summary_line = None ; let file_in = File :: open (file) . expect ("Unable to open cachegrind output file") ; for line in BufReader :: new (file_in) . lines () { let line = line . unwrap () ; if let Some (line) = line . strip_prefix ("events: ") { events_line = Some (line . trim () . to_owned ()) ; } if let Some (line) = line . strip_prefix ("summary: ") { summary_line = Some (line . trim () . to_owned ()) ; } } match (events_line , summary_line) { (Some (events) , Some (summary)) => { let events : HashMap < _ , _ > = events . split_whitespace () . zip (summary . split_whitespace () . map (| s | { s . parse :: < u64 > () . expect ("Unable to parse summary line from cachegrind output file") })) . collect () ; CachegrindStats { instruction_reads : events ["Ir"] , instruction_l1_misses : events ["I1mr"] , instruction_cache_misses : events ["ILmr"] , data_reads : events ["Dr"] , data_l1_read_misses : events ["D1mr"] , data_cache_read_misses : events ["DLmr"] , data_writes : events ["Dw"] , data_l1_write_misses : events ["D1mw"] , data_cache_write_misses : events ["DLmw"] , } } _ => panic ! ("Unable to parse cachegrind output file") , } }
};
}
