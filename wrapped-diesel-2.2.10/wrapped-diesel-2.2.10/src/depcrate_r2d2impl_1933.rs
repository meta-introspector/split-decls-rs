// Generated macro for impl_1933 (impl)
macro_rules! Depcrate_r2d2impl_1933 {
() => {
// Module: crate::r2d2
// Provides: {"impl_1933"}
// Dependencies: {}
impl < T > ConnectionManager < T > { # [doc = " Returns a new connection manager,"] # [doc = " which establishes connections to the given database URL."] pub fn new < S : Into < String > > (database_url : S) -> Self { ConnectionManager { database_url : database_url . into () , _marker : PhantomData , } } # [doc = " Modifies the URL which was supplied at initialization."] # [doc = ""] # [doc = " This does not update any state for existing connections,"] # [doc = " but this new URL is used for new connections that are created."] pub fn update_database_url < S : Into < String > > (& mut self , database_url : S) { self . database_url = database_url . into () ; } }
};
}
