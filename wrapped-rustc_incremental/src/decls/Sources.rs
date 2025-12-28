macro_rules! Sources {
    () => {
        type Sources = Vec < (Span , DefId , DepNode) > ;
    };
}

Sources!()