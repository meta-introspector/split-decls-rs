macro_rules! Info {
    () => {
        # [derive (Debug)] struct Info < 'tcx > { type_span : Span , referenced_type_span : Option < Span > , lifetime : & 'tcx hir :: Lifetime , }
    };
}

Info!();