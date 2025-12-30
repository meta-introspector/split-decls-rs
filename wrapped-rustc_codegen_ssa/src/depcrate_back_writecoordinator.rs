// Generated macro for Coordinator (struct)
macro_rules! Depcrate_back_writeCoordinator {
() => {
// Module: crate::back::write
// Provides: {"Coordinator"}
// Dependencies: {}
pub struct Coordinator < B : ExtraBackendMethods > { sender : Sender < Message < B > > , future : Option < thread :: JoinHandle < Result < CompiledModules , () > > > , phantom : PhantomData < B > , }
};
}
