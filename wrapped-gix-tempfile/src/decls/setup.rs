macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! setup {
    () => {
        deps!();
        # [doc = " Initialize signal handlers and other state to keep track of tempfiles, and **must be called before the first tempfile is created**,"] # [doc = " allowing to set the `mode` in which signal handlers are installed."] # [doc = ""] # [doc = " Only has an effect the first time it is called."] # [doc = ""] # [doc = " Note that it is possible to not call this function and instead call"] # [doc = " [`registry::cleanup_tempfiles_signal_safe()`][crate::registry::cleanup_tempfiles_signal_safe()]"] # [doc = " from a signal handler under the application's control."] pub fn setup (mode : handler :: Mode) { handler :: MODE . store (mode as usize , std :: sync :: atomic :: Ordering :: SeqCst) ; LazyLock :: force (& REGISTRY) ; }
    };
}

setup!()