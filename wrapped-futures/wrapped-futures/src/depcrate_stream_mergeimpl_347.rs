// Generated macro for impl_347 (impl)
macro_rules! Depcrate_stream_mergeimpl_347 {
() => {
// Module: crate::stream::merge
// Provides: {"impl_347"}
// Dependencies: {}
impl < S1 , S2 > Stream for Merge < S1 , S2 > where S1 : Stream , S2 : Stream < Error = S1 :: Error > { type Item = MergedItem < S1 :: Item , S2 :: Item > ; type Error = S1 :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < Self :: Item > , Self :: Error > { if let Some (e) = self . queued_error . take () { return Poll :: Err (e) ; } match self . stream1 . poll (task) { Poll :: Err (e) => Poll :: Err (e) , Poll :: NotReady => match self . stream2 . poll (task) { Poll :: Err (e) => Poll :: Err (e) , Poll :: NotReady => Poll :: NotReady , Poll :: Ok (Some (item2)) => Poll :: Ok (Some (MergedItem :: Second (item2))) , Poll :: Ok (None) => Poll :: NotReady , } , Poll :: Ok (Some (item1)) => match self . stream2 . poll (task) { Poll :: Err (e) => { self . queued_error = Some (e) ; Poll :: Ok (Some (MergedItem :: First (item1))) } Poll :: NotReady => Poll :: Ok (Some (MergedItem :: First (item1))) , Poll :: Ok (Some (item2)) => Poll :: Ok (Some (MergedItem :: Both (item1 , item2))) , Poll :: Ok (None) => Poll :: Ok (Some (MergedItem :: First (item1))) , } , Poll :: Ok (None) => match self . stream2 . poll (task) { Poll :: Err (e) => Poll :: Err (e) , Poll :: NotReady => Poll :: NotReady , Poll :: Ok (Some (item2)) => Poll :: Ok (Some (MergedItem :: Second (item2))) , Poll :: Ok (None) => Poll :: Ok (None) , } , } } fn schedule (& mut self , task : & mut Task) { self . stream1 . schedule (task) ; self . stream2 . schedule (task) ; } }
};
}
