macro_rules! deps {
    () => {
        ConstArg!();
        Visitor!();
    };
}

macro_rules! walk_const_param_default {
    () => {
        deps!();
        pub fn walk_const_param_default < 'v , V : Visitor < 'v > > (visitor : & mut V , ct : & 'v ConstArg < 'v > ,) -> V :: Result { visitor . visit_const_arg_unambig (ct) }
    };
}

walk_const_param_default!();