macro_rules! shell {
    () => {
        # [doc = " Return the shell that Git would use, the shell to execute commands from."] # [doc = ""] # [doc = " On Windows, this is the full path to `sh.exe` bundled with Git for Windows if we can find it."] # [doc = " If the bundled shell on Windows cannot be found, `sh.exe` is returned as the name of a shell,"] # [doc = " as it could possibly be found in `PATH`. On Unix it's `/bin/sh` as the POSIX-compatible shell."] # [doc = ""] # [doc = " Note that the returned path might not be a path on disk, if it is a fallback path or if the"] # [doc = " file was moved or deleted since the first time this function is called."] pub fn shell () -> & 'static OsStr { static PATH : LazyLock < OsString > = LazyLock :: new (| | { if cfg ! (windows) { auxiliary :: find_git_associated_windows_executable_with_fallback ("sh") } else { "/bin/sh" . into () } }) ; PATH . as_ref () }
    };
}

shell!()