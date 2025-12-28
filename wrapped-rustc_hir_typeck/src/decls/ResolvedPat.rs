macro_rules! deps {
    () => {
        ResolvedPatKind!();
    };
}

macro_rules! ResolvedPat {
    () => {
        deps!();
        # [doc = " When checking patterns containing paths, we need to know the path's resolution to determine"] # [doc = " whether to apply match ergonomics and implicitly dereference the scrutinee. For instance, when"] # [doc = " the `deref_patterns` feature is enabled and we're matching against a scrutinee of type"] # [doc = " `Cow<'a, Option<u8>>`, we insert an implicit dereference to allow the pattern `Some(_)` to type,"] # [doc = " but we must not dereference it when checking the pattern `Cow::Borrowed(_)`."] # [doc = ""] # [doc = " `ResolvedPat` contains the information from resolution needed to determine match ergonomics"] # [doc = " adjustments, and to finish checking the pattern once we know its adjusted type."] # [derive (Clone , Copy , Debug)] struct ResolvedPat < 'tcx > { # [doc = " The type of the pattern, to be checked against the type of the scrutinee after peeling. This"] # [doc = " is also used to avoid peeling the scrutinee's constructors (see the `Cow` example above)."] ty : Ty < 'tcx > , kind : ResolvedPatKind < 'tcx > , }
    };
}

ResolvedPat!()