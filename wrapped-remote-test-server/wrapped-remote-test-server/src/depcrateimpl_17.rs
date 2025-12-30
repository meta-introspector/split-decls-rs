// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl Config { pub fn default () -> Config { Config { verbose : false , sequential : false , batch : false , bind : if cfg ! (target_os = "android") || cfg ! (windows) { ([0 , 0 , 0 , 0] , 12345) . into () } else { ([10 , 0 , 2 , 15] , 12345) . into () } , } } pub fn parse_args () -> Config { let mut config = Config :: default () ; let args = env :: args () . skip (1) ; let mut next_is_bind = false ; for argument in args { match & argument [..] { bind if next_is_bind => { config . bind = t ! (bind . parse ()) ; next_is_bind = false ; } "--bind" => next_is_bind = true , "--sequential" => config . sequential = true , "--batch" => config . batch = true , "--verbose" | "-v" => config . verbose = true , "--help" | "-h" => { show_help () ; std :: process :: exit (0) ; } arg => panic ! ("unknown argument: {}, use `--help` for known arguments" , arg) , } } if next_is_bind { panic ! ("missing value for --bind") ; } config } }
};
}
