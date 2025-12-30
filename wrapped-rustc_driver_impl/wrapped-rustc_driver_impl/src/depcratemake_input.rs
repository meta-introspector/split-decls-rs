// Generated macro for make_input (function)
macro_rules! Depcratemake_input {
() => {
// Module: crate
// Provides: {"make_input"}
// Dependencies: {}
# [doc = " Extract input (string or file and optional path) from matches."] # [doc = " This handles reading from stdin if `-` is provided."] fn make_input (early_dcx : & EarlyDiagCtxt , free_matches : & [String]) -> Option < Input > { match free_matches { [] => None , [ifile] if ifile == "-" => { let mut input = String :: new () ; if io :: stdin () . read_to_string (& mut input) . is_err () { early_dcx . early_fatal ("couldn't read from stdin, as it did not contain valid UTF-8") ; } let name = match env :: var ("UNSTABLE_RUSTDOC_TEST_PATH") { Ok (path) => { let line = env :: var ("UNSTABLE_RUSTDOC_TEST_LINE") . expect ("when UNSTABLE_RUSTDOC_TEST_PATH is set \
                                    UNSTABLE_RUSTDOC_TEST_LINE also needs to be set" ,) ; let line = isize :: from_str_radix (& line , 10) . expect ("UNSTABLE_RUSTDOC_TEST_LINE needs to be an number") ; FileName :: doc_test_source_code (PathBuf :: from (path) , line) } Err (_) => FileName :: anon_source_code (& input) , } ; Some (Input :: Str { name , input }) } [ifile] => Some (Input :: File (PathBuf :: from (ifile))) , [ifile1 , ifile2 , ..] => early_dcx . early_fatal (format ! ("multiple input filenames provided (first two filenames are `{}` and `{}`)" , ifile1 , ifile2)) , } }
};
}
