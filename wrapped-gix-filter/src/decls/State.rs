macro_rules! deps {
    () => {
        Context!();
        Client!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [doc = " State required to handle `process` filters, which are running until all their work is done."] # [doc = ""] # [doc = " These can be significantly faster on some platforms as they are launched only once, while supporting asynchronous processing."] # [doc = ""] # [doc = " ### Lifecycle"] # [doc = ""] # [doc = " Note that [`shutdown()`][State::shutdown()] must be called to finalize long-running processes."] # [doc = " Failing to do so will naturally shut them down by terminating their pipes, but finishing explicitly"] # [doc = " allows to wait for processes as well."] # [derive (Default)] pub struct State { # [doc = " The list of currently running processes. These are preferred over simple clean-and-smudge programs."] # [doc = ""] # [doc = " Note that these processes are expected to shut-down once their stdin/stdout are dropped, so nothing else"] # [doc = " needs to be done to clean them up after drop."] running : HashMap < BString , process :: Client > , # [doc = " The context to pass to spawned filter programs."] pub context : gix_command :: Context , }
    };
}

State!()