macro_rules! deps {
    () => {
        MaybeOwner!();
    };
}

macro_rules! Crate {
    () => {
        deps!();
        # [doc = " The top-level data structure that stores the entire contents of"] # [doc = " the crate currently being compiled."] # [doc = ""] # [doc = " For more details, see the [rustc dev guide]."] # [doc = ""] # [doc = " [rustc dev guide]: https://rustc-dev-guide.rust-lang.org/hir.html"] # [derive (Debug)] pub struct Crate < 'hir > { pub owners : IndexVec < LocalDefId , MaybeOwner < 'hir > > , pub opt_hir_hash : Option < Fingerprint > , }
    };
}

Crate!()