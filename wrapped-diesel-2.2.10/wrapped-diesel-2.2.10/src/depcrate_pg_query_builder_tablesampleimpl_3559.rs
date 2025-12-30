// Generated macro for impl_3559 (impl)
macro_rules! Depcrate_pg_query_builder_tablesampleimpl_3559 {
() => {
// Module: crate::pg::query_builder::tablesample
// Provides: {"impl_3559"}
// Dependencies: {}
impl < S , TSM > Tablesample < S , TSM > where TSM : TablesampleMethod , { pub (crate) fn new (source : S , portion : i16) -> Tablesample < S , TSM > { Tablesample { source , method : PhantomData , portion , seed : None , } } # [doc = " This method allows you to specify the random number generator seed to use in the sampling"] # [doc = " method. This allows you to obtain repeatable results."] pub fn with_seed (self , seed : f64) -> Tablesample < S , TSM > { Tablesample { source : self . source , method : self . method , portion : self . portion , seed : Some (seed) , } } }
};
}
