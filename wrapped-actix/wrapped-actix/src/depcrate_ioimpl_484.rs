// Generated macro for impl_484 (impl)
macro_rules! Depcrate_ioimpl_484 {
() => {
// Module: crate::io
// Provides: {"impl_484"}
// Dependencies: {}
impl < I : 'static , S : Sink < I > + Unpin + 'static > SinkWrite < I , S > { pub fn new < A , C > (sink : S , ctxt : & mut C) -> Self where A : Actor < Context = C > + WriteHandler < S :: Error > , C : AsyncContext < A > , { let inner = Rc :: new (RefCell :: new (InnerSinkWrite { _i : PhantomData , closing_flag : Flags :: empty () , sink , task : None , handle : SpawnHandle :: default () , buffer : VecDeque :: new () , })) ; let handle = ctxt . spawn (SinkWriteFuture { inner : inner . clone () , }) ; inner . borrow_mut () . handle = handle ; SinkWrite { inner } } # [doc = " Queues an item to be sent to the sink."] # [doc = ""] # [doc = " Returns unsent item if sink is closing or closed."] pub fn write (& mut self , item : I) -> Result < () , I > { if self . inner . borrow () . closing_flag . is_empty () { self . inner . borrow_mut () . buffer . push_back (item) ; self . notify_task () ; Ok (()) } else { Err (item) } } # [doc = " Gracefully closes the sink."] # [doc = ""] # [doc = " The closing happens asynchronously."] pub fn close (& mut self) { self . inner . borrow_mut () . closing_flag . insert (Flags :: CLOSING) ; self . notify_task () ; } # [doc = " Checks if the sink is closed."] pub fn closed (& self) -> bool { self . inner . borrow_mut () . closing_flag . contains (Flags :: CLOSED) } fn notify_task (& self) { if let Some (task) = & self . inner . borrow () . task { task . wake_by_ref () } } # [doc = " Returns the `SpawnHandle` for this writer."] pub fn handle (& self) -> SpawnHandle { self . inner . borrow () . handle } }
};
}
