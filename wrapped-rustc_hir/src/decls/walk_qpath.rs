macro_rules! deps {
    () => {
        Visitor!();
        QPath!();
    };
}

macro_rules! walk_qpath {
    () => {
        deps!();
        pub fn walk_qpath < 'v , V : Visitor < 'v > > (visitor : & mut V , qpath : & 'v QPath < 'v > , id : HirId ,) -> V :: Result { match * qpath { QPath :: Resolved (ref maybe_qself , ref path) => { visit_opt ! (visitor , visit_ty_unambig , maybe_qself) ; visitor . visit_path (path , id) } QPath :: TypeRelative (ref qself , ref segment) => { try_visit ! (visitor . visit_ty_unambig (qself)) ; visitor . visit_path_segment (segment) } QPath :: LangItem (..) => V :: Result :: output () , } }
    };
}

walk_qpath!()