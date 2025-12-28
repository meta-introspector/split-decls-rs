macro_rules! deps {
    () => {
        Body!();
        Visitor!();
    };
}

macro_rules! walk_body {
    () => {
        deps!();
        pub fn walk_body < 'v , V : Visitor < 'v > > (visitor : & mut V , body : & Body < 'v >) -> V :: Result { let Body { params , value } = body ; walk_list ! (visitor , visit_param , * params) ; visitor . visit_expr (* value) }
    };
}

walk_body!()