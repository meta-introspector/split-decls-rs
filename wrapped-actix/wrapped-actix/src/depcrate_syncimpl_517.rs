// Generated macro for impl_517 (impl)
macro_rules! Depcrate_syncimpl_517 {
() => {
// Module: crate::sync
// Provides: {"impl_517"}
// Dependencies: {}
impl < A > SyncContext < A > where A : Actor < Context = Self > , { fn new (factory : Arc < dyn Fn () -> A > , queue : cb_channel :: Receiver < Envelope < A > > , address : AddressSenderProducer < A > ,) -> Self { let act = factory () ; Self { queue , factory , act : Some (act) , stopping : false , state : ActorState :: Started , address , } } fn run (& mut self) { let mut act = self . act . take () . unwrap () ; A :: started (& mut act , self) ; self . state = ActorState :: Running ; loop { match self . queue . recv () { Ok (mut env) => { env . handle (& mut act , self) ; } Err (_) => { self . state = ActorState :: Stopping ; if A :: stopping (& mut act , self) != Running :: Stop { warn ! ("stopping method is not supported for sync actors") ; } self . state = ActorState :: Stopped ; A :: stopped (& mut act , self) ; return ; } } if self . stopping { self . stopping = false ; A :: stopping (& mut act , self) ; self . state = ActorState :: Stopped ; A :: stopped (& mut act , self) ; self . state = ActorState :: Started ; act = (* self . factory) () ; A :: started (& mut act , self) ; self . state = ActorState :: Running ; } } } pub fn address (& self) -> Addr < A > { Addr :: new (self . address . sender ()) } }
};
}
