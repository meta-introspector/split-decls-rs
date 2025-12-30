// Generated macro for read_stats_from_file (function)
macro_rules! Depcrate_outputread_stats_from_file {
() => {
// Module: crate::output
// Provides: {"read_stats_from_file"}
// Dependencies: {}
# [doc = " read the previous stats from the lintcheck-log file"] fn read_stats_from_file (file_path : & Path) -> HashMap < String , usize > { let file_content : String = match fs :: read_to_string (file_path) . ok () { Some (content) => content , None => { return HashMap :: new () ; } , } ; let lines : Vec < String > = file_content . lines () . map (ToString :: to_string) . collect () ; lines . iter () . skip_while (| line | line . as_str () != "### Stats:") . skip (4) . take_while (| line | line . starts_with ("| ")) . filter_map (| line | { let mut spl = line . split ('|') ; spl . next () ; if let (Some (lint) , Some (count)) = (spl . next () , spl . next ()) { Some ((lint . trim () . to_string () , count . trim () . parse :: < usize > () . unwrap ())) } else { None } }) . collect :: < HashMap < String , usize > > () }
};
}
