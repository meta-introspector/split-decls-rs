macro_rules! deps {
    () => {
        Clone!();
    };
}

macro_rules! WritePackedRefs {
    () => {
        deps!();
        # [doc = " How to deal with refs when cloning or fetching."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg (any (feature = "blocking-network-client" , feature = "async-network-client"))] pub (crate) enum WritePackedRefs { # [doc = " Normal operation, i.e. don't use packed-refs at all for writing."] Never , # [doc = " Put ref updates straight into the `packed-refs` file, without creating loose refs first or dealing with them in any way."] Only , }
    };
}

WritePackedRefs!();