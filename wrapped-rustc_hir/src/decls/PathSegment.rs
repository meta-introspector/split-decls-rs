macro_rules! deps {
    () => {
        Res!();
        GenericArgs!();
    };
}

macro_rules! PathSegment {
    () => {
        deps!();
        # [doc = " A segment of a path: an identifier, an optional lifetime, and a set of"] # [doc = " types."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct PathSegment < 'hir > { # [doc = " The identifier portion of this path segment."] pub ident : Ident , # [stable_hasher (ignore)] pub hir_id : HirId , pub res : Res , # [doc = " Type/lifetime parameters attached to this path. They come in"] # [doc = " two flavors: `Path<A,B,C>` and `Path(A,B) -> C`. Note that"] # [doc = " this is more than just simple syntactic sugar; the use of"] # [doc = " parens affects the region binding rules, so we preserve the"] # [doc = " distinction."] pub args : Option < & 'hir GenericArgs < 'hir > > , # [doc = " Whether to infer remaining type parameters, if any."] # [doc = " This only applies to expression and pattern paths, and"] # [doc = " out of those only the segments with no type parameters"] # [doc = " to begin with, e.g., `Vec::new` is `<Vec<..>>::new::<..>`."] pub infer_args : bool , }
    };
}

PathSegment!()