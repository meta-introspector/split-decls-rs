// Generated macro for Context (struct)
macro_rules! Depcrate_interact_contextContext {
() => {
// Module: crate::interact::context
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Context provides an interface to use a [`Session`], IO streams"] # [doc = " and a state."] # [doc = ""] # [doc = " It's used primarily in callbacks for [`InteractSession`]."] # [doc = ""] # [doc = " [`InteractSession`]: crate::interact::InteractSession"] # [doc = " [`Session`]: crate::session::Session"] # [derive (Debug)] pub struct Context < 'a , Session , Input , Output , State > { # [doc = " The field contains a &mut reference to a [`Session`]."] # [doc = ""] # [doc = " [`Session`]: crate::session::Session"] pub session : & 'a mut Session , # [doc = " The field contains an input structure which was used in [`InteractSession`]."] # [doc = ""] # [doc = " [`InteractSession`]: crate::interact::InteractSession"] pub input : & 'a mut Input , # [doc = " The field contains an output structure which was used in [`InteractSession`]."] # [doc = ""] # [doc = " [`InteractSession`]: crate::interact::InteractSession"] pub output : & 'a mut Output , # [doc = " The field contains a user defined data."] pub state : & 'a mut State , # [doc = " The field contains a bytes which were consumed from a user or the running process."] pub buf : & 'a [u8] , # [doc = " A flag for EOF of a user session or running process."] pub eof : bool , }
};
}
