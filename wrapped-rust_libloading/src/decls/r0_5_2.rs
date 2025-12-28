macro_rules! deps {
    () => {
        Library!();
        Symbol!();
    };
}

macro_rules! r0_5_2 {
    () => {
        deps!();
        # [doc = " Release 0.5.2 (2019-07-07)"] # [doc = ""] # [doc = " * Added API to convert OS-specific `Library` and `Symbol` conversion to underlying resources."] pub mod r0_5_2 { }
    };
}

r0_5_2!()