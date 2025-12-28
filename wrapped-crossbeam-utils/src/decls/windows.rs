macro_rules! deps {
    () => {
        ScopedJoinHandle!();
    };
}

macro_rules! windows {
    () => {
        deps!();
        # [doc = " Windows-specific extensions."] # [cfg (windows)] mod windows { use super :: ScopedJoinHandle ; use std :: os :: windows :: io :: { AsRawHandle , IntoRawHandle , RawHandle } ; impl < T > AsRawHandle for ScopedJoinHandle < '_ , T > { fn as_raw_handle (& self) -> RawHandle { let handle = self . handle . lock () . unwrap () ; handle . as_ref () . unwrap () . as_raw_handle () } } impl < T > IntoRawHandle for ScopedJoinHandle < '_ , T > { fn into_raw_handle (self) -> RawHandle { self . as_raw_handle () } } }
    };
}

windows!();