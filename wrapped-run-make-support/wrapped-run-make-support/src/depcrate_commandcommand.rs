// Generated macro for Command (struct)
macro_rules! Depcrate_commandCommand {
() => {
// Module: crate::command
// Provides: {"Command"}
// Dependencies: {}
# [doc = " This is a custom command wrapper that simplifies working with commands and makes it easier to"] # [doc = " ensure that we check the exit status of executed processes."] # [doc = ""] # [doc = " # A [`Command`] must be executed exactly once"] # [doc = ""] # [doc = " A [`Command`] is armed by a [`DropBomb`] on construction to enforce that it will be executed. If"] # [doc = " a [`Command`] is constructed but never executed, the drop bomb will explode and cause the test"] # [doc = " to panic. Execution methods [`run`] and [`run_fail`] will defuse the drop bomb. A test"] # [doc = " containing constructed but never executed commands is dangerous because it can give a false"] # [doc = " sense of confidence."] # [doc = ""] # [doc = " Each [`Command`] invocation can also only be executed once, because we want to enforce"] # [doc = " `std{in,out,err}` config via [`std::process::Stdio`] but [`std::process::Stdio`] is not"] # [doc = " cloneable."] # [doc = ""] # [doc = " In this sense, [`Command`] exhibits linear type semantics but enforced at run-time."] # [doc = ""] # [doc = " [`run`]: Self::run"] # [doc = " [`run_fail`]: Self::run_fail"] # [doc = " [`run_unchecked`]: Self::run_unchecked"] # [derive (Debug)] pub struct Command { cmd : StdCommand , stdin_buf : Option < Box < [u8] > > , stdin : Option < Stdio > , stdout : Option < Stdio > , stderr : Option < Stdio > , drop_bomb : DropBomb , already_executed : bool , }
};
}
