// Generated macro for impl_220 (impl)
macro_rules! Depcrateimpl_220 {
() => {
// Module: crate
// Provides: {"impl_220"}
// Dependencies: {}
impl fmt :: Display for VersionError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { VersionError :: Exec (err) => write ! (f , "`gnuplot --version` failed: {}" , err) , VersionError :: Error (msg) => { write ! (f , "`gnuplot --version` failed with error message:\n{}" , msg) } VersionError :: OutputError => write ! (f , "`gnuplot --version` returned invalid utf-8") , VersionError :: ParseError (msg) => write ! (f , "`gnuplot --version` returned an unparsable version string: {}" , msg) , } } }
};
}
