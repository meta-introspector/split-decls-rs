// Generated macro for macro_24 (macro)
macro_rules! Depcratemacro_24 {
() => {
// Module: crate
// Provides: {"macro_24"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " Multi purpose, shared flags that are used by negotiation algorithms and by the caller as well"] # [doc = ""] # [doc = " However, in this crate we can't implement the calling side, so we marry this type to whatever is needed in downstream crates."] # [derive (Debug , Default , Copy , Clone)] pub struct Flags : u8 { # [doc = " The object is already available locally and doesn't need to be fetched by the remote."] const COMPLETE = 1 << 0 ; # [doc = " A commit from an alternate object database."] const ALTERNATE = 1 << 1 ; # [doc = " The revision is known to be in common with the remote."] # [doc = ""] # [doc = " Used by `consecutive` and `skipping`."] const COMMON = 1 << 2 ; # [doc = " The revision has entered the priority queue."] # [doc = ""] # [doc = " Used by `consecutive` and `skipping`."] const SEEN = 1 << 3 ; # [doc = " The revision was popped off our primary priority queue, used to avoid double-counting of `non_common_revs`"] # [doc = ""] # [doc = " Used by `consecutive` and `skipping`."] const POPPED = 1 << 4 ; # [doc = " The revision is common and was set by merit of a remote tracking ref (e.g. `refs/heads/origin/main`)."] # [doc = ""] # [doc = " Used by `consecutive`."] const COMMON_REF = 1 << 5 ; # [doc = " The remote let us know it has the object. We still have to tell the server we have this object or one of its descendants."] # [doc = " We won't tell the server about its ancestors."] # [doc = ""] # [doc = " Used by `skipping`."] const ADVERTISED = 1 << 6 ; } }
};
}
