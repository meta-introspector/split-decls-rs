macro_rules! deps {
    () => {
        Visitor!();
        PathSegment!();
    };
}

macro_rules! walk_path_segment {
    () => {
        deps!();
        pub fn walk_path_segment < 'v , V : Visitor < 'v > > (visitor : & mut V , segment : & 'v PathSegment < 'v > ,) -> V :: Result { let PathSegment { ident , hir_id , res : _ , args , infer_args : _ } = segment ; try_visit ! (visitor . visit_ident (* ident)) ; try_visit ! (visitor . visit_id (* hir_id)) ; visit_opt ! (visitor , visit_generic_args , * args) ; V :: Result :: output () }
    };
}

walk_path_segment!();