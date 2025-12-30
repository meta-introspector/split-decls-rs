// Generated macro for factory (function)
macro_rules! Depcratefactory {
() => {
// Module: crate
// Provides: {"factory"}
// Dependencies: {}
fn factory (remote : & git2 :: Remote < '_ > , handle : Arc < Mutex < Easy > >) -> Result < Transport , Error > { Transport :: smart (remote , true , CurlTransport { handle : handle , base_url : Arc :: new (Mutex :: new (String :: new ())) , } ,) }
};
}
