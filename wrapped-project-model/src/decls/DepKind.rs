macro_rules! deps {
    () => {
        Build!();
    };
}

macro_rules! DepKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum DepKind { # [doc = " Available to the library, binary, and dev targets in the package (but not the build script)."] Normal , # [doc = " Available only to test and bench targets (and the library target, when built with `cfg(test)`)."] Dev , # [doc = " Available only to the build script target."] Build , }
    };
}

DepKind!();