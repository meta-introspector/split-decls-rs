// Generated macro for impl_48 (impl)
macro_rules! Depcrate_confimpl_48 {
() => {
// Module: crate::conf
// Provides: {"impl_48"}
// Dependencies: {}
impl Conf { pub fn read (sess : & Session , path : & io :: Result < (Option < PathBuf > , Vec < String >) >) -> & 'static Conf { static CONF : OnceLock < Conf > = OnceLock :: new () ; CONF . get_or_init (| | Conf :: read_inner (sess , path)) } fn read_inner (sess : & Session , path : & io :: Result < (Option < PathBuf > , Vec < String >) >) -> Conf { match path { Ok ((_ , warnings)) => { for warning in warnings { sess . dcx () . warn (warning . clone ()) ; } } , Err (error) => { sess . dcx () . err (format ! ("error finding Clippy's configuration file: {error}")) ; } , } let TryConf { mut conf , value_spans : _ , errors , warnings , } = match path { Ok ((Some (path) , _)) => match sess . source_map () . load_file (path) { Ok (file) => deserialize (& file) , Err (error) => { sess . dcx () . err (format ! ("failed to read `{}`: {error}" , path . display ())) ; TryConf :: default () } , } , _ => TryConf :: default () , } ; conf . msrv . read_cargo (sess) ; for error in errors { let mut diag = sess . dcx () . struct_span_err (error . span , format ! ("error reading Clippy's configuration file: {}" , error . message) ,) ; if let Some (sugg) = error . suggestion { diag . span_suggestion (error . span , sugg . message , sugg . suggestion , Applicability :: MaybeIncorrect) ; } diag . emit () ; } for warning in warnings { sess . dcx () . span_warn (warning . span , format ! ("error reading Clippy's configuration file: {}" , warning . message) ,) ; } conf } }
};
}
