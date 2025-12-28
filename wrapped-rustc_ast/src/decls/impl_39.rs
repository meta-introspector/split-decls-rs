macro_rules! deps {
    () => {
        WhereClause!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl WhereClause { pub fn is_empty (& self) -> bool { ! self . has_where_token && self . predicates . is_empty () } }
    };
}

impl_39!()