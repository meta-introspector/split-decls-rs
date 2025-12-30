// Generated macro for impl_279 (impl)
macro_rules! Depcrate_actors_mockerimpl_279 {
() => {
// Module: crate::actors::mocker
// Provides: {"impl_279"}
// Dependencies: {}
impl < M : 'static , T : Sized + Unpin + 'static > Handler < M > for Mocker < T > where M : Message , < M as Message > :: Result : MessageResponse < Mocker < T > , M > , { type Result = M :: Result ; fn handle (& mut self , msg : M , ctx : & mut Self :: Context) -> M :: Result { let mut ret = (self . mock) (Box :: new (msg) , ctx) ; let out = ret . downcast_mut :: < Option < M :: Result > > () . expect ("wrong return type for message") . take () ; match out { Some (a) => a , _ => panic ! () , } } }
};
}
