macro_rules! deps {
    () => {
        RegionNameSource!();
    };
}

macro_rules! RegionName {
    () => {
        deps!();
        # [doc = " A name for a particular region used in emitting diagnostics. This name could be a generated"] # [doc = " name like `'1`, a name used by the user like `'a`, or a name like `'static`."] # [derive (Debug , Clone , Copy)] pub (crate) struct RegionName { # [doc = " The name of the region (interned)."] pub (crate) name : Symbol , # [doc = " Where the region comes from."] pub (crate) source : RegionNameSource , }
    };
}

RegionName!();