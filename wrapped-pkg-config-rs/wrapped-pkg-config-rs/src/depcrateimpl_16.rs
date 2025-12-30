// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl WrappedCommand { fn new < S : AsRef < OsStr > > (program : S) -> Self { Self { inner : Command :: new (program . as_ref ()) , program : program . as_ref () . to_os_string () , env_vars : Vec :: new () , args : Vec :: new () , } } fn args < I , S > (& mut self , args : I) -> & mut Self where I : IntoIterator < Item = S > + Clone , S : AsRef < OsStr > , { self . inner . args (args . clone ()) ; self . args . extend (args . into_iter () . map (| arg | arg . as_ref () . to_os_string ())) ; self } fn arg < S : AsRef < OsStr > > (& mut self , arg : S) -> & mut Self { self . inner . arg (arg . as_ref ()) ; self . args . push (arg . as_ref () . to_os_string ()) ; self } fn env < K , V > (& mut self , key : K , value : V) -> & mut Self where K : AsRef < OsStr > , V : AsRef < OsStr > , { self . inner . env (key . as_ref () , value . as_ref ()) ; self . env_vars . push ((key . as_ref () . to_os_string () , value . as_ref () . to_os_string ())) ; self } fn output (& mut self) -> io :: Result < Output > { self . inner . output () } }
};
}
