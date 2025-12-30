// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl IBackgroundCopyCallback_Impl for Callback_Impl { fn JobTransferred (& self , job : Ref < IBackgroundCopyJob >) -> Result < () > { let job = job . unwrap () ; unsafe { job . Complete () ? } ; println ! ("done") ; std :: process :: exit (0) ; } fn JobError (& self , job : Ref < IBackgroundCopyJob > , error : Ref < IBackgroundCopyError > ,) -> Result < () > { let job = job . unwrap () ; let error = error . unwrap () ; unsafe { job . Cancel () ? ; println ! ("{}" , error . GetErrorDescription (0) ?. display ()) ; } std :: process :: exit (0) ; } fn JobModification (& self , _ : Ref < IBackgroundCopyJob > , _ : u32) -> Result < () > { Ok (()) } }
};
}
