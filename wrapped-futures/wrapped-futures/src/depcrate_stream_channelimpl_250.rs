// Generated macro for impl_250 (impl)
macro_rules! Depcrate_stream_channelimpl_250 {
() => {
// Module: crate::stream::channel
// Provides: {"impl_250"}
// Dependencies: {}
impl < T , E > Stream for Receiver < T , E > where T : Send + 'static , E : Send + 'static , { type Item = T ; type Error = E ; fn poll (& mut self , _task : & mut Task) -> Poll < Option < T > , E > { match self . inner . slot . try_consume () { Ok (Message :: Data (Ok (e))) => Poll :: Ok (Some (e)) , Ok (Message :: Data (Err (e))) => Poll :: Err (e) , Ok (Message :: Done) => Poll :: Ok (None) , Err (..) => Poll :: NotReady , } } fn schedule (& mut self , task : & mut Task) { if let Some (token) = self . on_full_token . take () { self . inner . slot . cancel (token) ; } let handle = task . handle () . clone () ; self . on_full_token = Some (self . inner . slot . on_full (move | _ | { handle . notify () ; })) ; } }
};
}
