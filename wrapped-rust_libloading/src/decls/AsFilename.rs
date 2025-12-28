macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! AsFilename {
    () => {
        deps!();
        # [doc = " This trait is implemented for types that can be used as a filename when loading new"] # [doc = " [`Library`](crate::Library) instances."] # [doc = ""] # [doc = " It is currently sealed and cannot be implemented or its methods called by users of this crate."] # [expect (private_bounds)] pub trait AsFilename : Sealed { }
    };
}

AsFilename!()