macro_rules! deps {
    () => {
        UnsupportedLinkSelfContained!();
    };
}

macro_rules! self_contained_components {
    () => {
        deps!();
        # [doc = " Various toolchain components used during linking are used from rustc distribution"] # [doc = " instead of being found somewhere on the host system."] # [doc = " We only provide such support for a very limited number of targets."] fn self_contained_components (sess : & Session , crate_type : CrateType , linker : & Path ,) -> LinkSelfContainedComponents { let self_contained = if let Some (self_contained) = sess . opts . cg . link_self_contained . explicitly_set { if sess . target . link_self_contained . is_disabled () { sess . dcx () . emit_err (errors :: UnsupportedLinkSelfContained) ; } self_contained } else { match sess . target . link_self_contained { LinkSelfContainedDefault :: False => false , LinkSelfContainedDefault :: True => true , LinkSelfContainedDefault :: WithComponents (components) => { return components ; } LinkSelfContainedDefault :: InferredForMusl => sess . crt_static (Some (crate_type)) , LinkSelfContainedDefault :: InferredForMingw => { sess . host == sess . target && sess . target . vendor != "uwp" && detect_self_contained_mingw (sess , linker) } } } ; if self_contained { LinkSelfContainedComponents :: all () } else { LinkSelfContainedComponents :: empty () } }
    };
}

self_contained_components!()