// Generated macro for impl_108 (impl)
macro_rules! Depcrate_flycheckimpl_108 {
() => {
// Module: crate::flycheck
// Provides: {"impl_108"}
// Dependencies: {}
impl FlycheckHandle { pub (crate) fn spawn (id : usize , sender : Sender < FlycheckMessage > , config : FlycheckConfig , sysroot_root : Option < AbsPathBuf > , workspace_root : AbsPathBuf , manifest_path : Option < AbsPathBuf > ,) -> FlycheckHandle { let actor = FlycheckActor :: new (id , sender , config , sysroot_root , workspace_root , manifest_path) ; let (sender , receiver) = unbounded :: < StateChange > () ; let thread = stdx :: thread :: Builder :: new (stdx :: thread :: ThreadIntent :: Worker , format ! ("Flycheck{id}")) . spawn (move | | actor . run (receiver)) . expect ("failed to spawn thread") ; FlycheckHandle { id , sender , _thread : thread } } # [doc = " Schedule a re-start of the cargo check worker to do a workspace wide check."] pub (crate) fn restart_workspace (& self , saved_file : Option < AbsPathBuf >) { self . sender . send (StateChange :: Restart { package : None , saved_file , target : None }) . unwrap () ; } # [doc = " Schedule a re-start of the cargo check worker to do a package wide check."] pub (crate) fn restart_for_package (& self , package : String , target : Option < Target >) { self . sender . send (StateChange :: Restart { package : Some (package) , saved_file : None , target }) . unwrap () ; } # [doc = " Stop this cargo check worker."] pub (crate) fn cancel (& self) { self . sender . send (StateChange :: Cancel) . unwrap () ; } pub (crate) fn id (& self) -> usize { self . id } }
};
}
