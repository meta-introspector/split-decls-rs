macro_rules! deps {
    () => {
        Match!();
        MatchFinder!();
    };
}

macro_rules! print_match_debug_info {
    () => {
        deps!();
        # [allow (clippy :: print_stdout)] fn print_match_debug_info (match_finder : & MatchFinder < '_ > , file_id : EditionedFileId , snippet : & str) { let debug_info = match_finder . debug_where_text_equal (file_id , snippet) ; println ! ("Match debug info: {} nodes had text exactly equal to '{}'" , debug_info . len () , snippet) ; for (index , d) in debug_info . iter () . enumerate () { println ! ("Node #{index}\n{d:#?}\n") ; } }
    };
}

print_match_debug_info!();