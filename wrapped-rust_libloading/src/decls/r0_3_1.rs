macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! r0_3_1 {
    () => {
        deps!();
        # [doc = " Release 0.3.1 (2016-10-01)"] # [doc = ""] # [doc = " * `Symbol<T>` and `os::*::Symbol<T>` now implement `Send` where `T: Send`;"] # [doc = " * `Symbol<T>` and `os::*::Symbol<T>` now implement `Sync` where `T: Sync`;"] # [doc = " * `Library` and `os::*::Library` now implement `Sync` (they were `Send` in 0.3.0 already)."] pub mod r0_3_1 { }
    };
}

r0_3_1!()