macro_rules! deps {
    () => {
        VisitorContext!();
    };
}

macro_rules! validate_type {
    () => {
        deps!();
        fn validate_type (ctx : & mut VisitorContext < '_ > , type_name : & str , pos : Pos) { if ! ctx . registry . types . contains_key (type_name) { ctx . report_error (vec ! [pos] , format ! (r#"Unknown type "{}""# , type_name)) ; } }
    };
}

validate_type!();