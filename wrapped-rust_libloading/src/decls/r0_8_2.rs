macro_rules! r0_8_2 {
    () => {
        # [doc = " Release 0.8.2 (2024-03-01)"] # [doc = ""] # [doc = " ## (Potentially) breaking changes"] # [doc = ""] # [doc = " MSRV has been increased to 1.56.0. Since both rustc versions are ancient, this has been deemed"] # [doc = " to not be breaking enough to warrant a semver-breaking release of libloading. If you're stick"] # [doc = " with a version of rustc older than 1.56.0, lock `libloading` dependency to `0.8.1`."] # [doc = ""] # [doc = " ## Non-breaking changes"] # [doc = ""] # [doc = " * The crate switches the dependency on `windows-sys` to a `windows-target` one for Windows"] # [doc = "   bindings. In order to enable this `libloading` defines any bindings necessary for its operation"] # [doc = "   internally, just like has been done for `unix` targets. This should result in leaner dependency"] # [doc = "   trees."] # [doc = " * `os::unix::with_dlerror` has been exposed for the users who need to invoke `dl*` family of"] # [doc = "   functions manually."] pub mod r0_8_2 { }
    };
}

r0_8_2!()