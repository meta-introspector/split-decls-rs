macro_rules! subty_if {
    () => {
        fn subty_if < F > (ty : & Type , f : F) -> Option < & Type > where F : FnOnce (& PathSegment) -> bool , { only_last_segment (ty) . filter (| segment | f (segment)) . and_then (| segment | { if let AngleBracketed (args) = & segment . arguments { only_one (args . args . iter ()) . and_then (| genneric | { if let GenericArgument :: Type (ty) = genneric { Some (ty) } else { None } }) } else { None } }) }
    };
}

subty_if!()