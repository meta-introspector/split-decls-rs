// Generated macro for impl_117 (impl)
macro_rules! Depcrate_interact_sessionimpl_117 {
() => {
// Module: crate::interact::session
// Provides: {"impl_117"}
// Dependencies: {}
impl < S , I , O , C > InteractSession < S , I , O , C > { # [doc = " Default escape character. <Ctrl-\\]>"] pub const ESCAPE : u8 = 29 ; # [doc = " Creates a new object of [`InteractSession`]."] pub fn new (session : S , input : I , output : O , state : C) -> InteractSession < S , I , O , C > { InteractSession { input , output , session , escape_character : Self :: ESCAPE , opts : InteractOptions { state , input_filter : None , output_filter : None , input_action : None , output_action : None , idle_action : None , } , # [cfg (unix)] status : None , } } # [doc = " Sets an escape character after seen which the interact interactions will be stopped"] # [doc = " and controll will be returned to a caller process."] pub fn set_escape_character (mut self , c : u8) -> Self { self . escape_character = c ; self } # [doc = " Returns a status of spawned session if it was exited."] # [doc = ""] # [doc = " If [`Self::spawn`] returns false but this method returns None it means that a child process was shutdown by various reasons."] # [doc = " Which sometimes happens and it's not considered to be a valid [`WaitStatus`], so None is returned."] # [doc = ""] # [doc = " [`Self::spawn`]: crate::interact::InteractSession::spawn"] # [cfg (unix)] pub fn get_status (& self) -> Option < WaitStatus > { self . status } }
};
}
