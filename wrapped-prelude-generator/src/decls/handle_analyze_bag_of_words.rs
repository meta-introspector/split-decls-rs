macro_rules! deps {
    () => {
        Args!();
        BagOfWordsVisitor!();
    };
}

macro_rules! handle_analyze_bag_of_words {
    () => {
        deps!();
        pub fn handle_analyze_bag_of_words (_project_root : & PathBuf , args : & crate :: Args ,) -> anyhow :: Result < () > { println ! (r"Analyzing bag of words...") ; let project_root = if args . path == PathBuf :: from (".") { std :: env :: current_dir () ? . parent () . unwrap () . to_path_buf () } else { PathBuf :: from (& args . path) } ; let mut bag_of_words_visitor = BagOfWordsVisitor :: new () ; let mut files_processed_for_bow = 0 ; for entry in walkdir :: WalkDir :: new (& project_root) . into_iter () . filter_map (| e | e . ok ()) . filter (| e | { e . file_type () . is_file () && e . path () . extension () . map_or (false , | ext | ext == r"rs") }) { let path = entry . path () ; if let Ok (content) = std :: fs :: read_to_string (& path) { if let Ok (file) = syn :: parse_file (& content) { bag_of_words_visitor . visit_file (& file) ; files_processed_for_bow += 1 ; } else { eprintln ! (r"Warning: Could not parse file for bag of words analysis: {}" , path . display ()) ; } } else { eprintln ! (r"Warning: Could not read file for bag of words analysis: {}" , path . display ()) ; } } println ! (r"Processed {} files for bag of words analysis." , files_processed_for_bow) ; println ! (r"Top 20 most common terms:") ; let mut sorted_terms : Vec < (& String , & usize) > = bag_of_words_visitor . bag_of_words . iter () . collect () ; sorted_terms . sort_by (| a , b | b . 1 . cmp (a . 1)) ; for (term , count) in sorted_terms . iter () . take (20) { println ! (r"  - {}: {}" , term , count) ; } Ok (()) }
    };
}

handle_analyze_bag_of_words!()