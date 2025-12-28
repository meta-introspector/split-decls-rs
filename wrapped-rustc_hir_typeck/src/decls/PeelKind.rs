macro_rules! deps {
    () => {
        ResolvedPat!();
    };
}

macro_rules! PeelKind {
    () => {
        deps!();
        # [doc = " Restrictions on what types to peel when adjusting the expected type and binding mode."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] enum PeelKind { # [doc = " Only peel reference types. This is used for explicit `deref!(_)` patterns, which dereference"] # [doc = " any number of `&`/`&mut` references, plus a single smart pointer."] ExplicitDerefPat , # [doc = " Implicitly peel references, and if `deref_patterns` is enabled, smart pointer ADTs."] Implicit { # [doc = " The ADT the pattern is a constructor for, if applicable, so that we don't peel it. See"] # [doc = " [`ResolvedPat`] for more information."] until_adt : Option < DefId > , # [doc = " The number of references at the head of the pattern's type, so we can leave that many"] # [doc = " untouched. This is `1` for string literals, and `0` for most patterns."] pat_ref_layers : usize , } , }
    };
}

PeelKind!()