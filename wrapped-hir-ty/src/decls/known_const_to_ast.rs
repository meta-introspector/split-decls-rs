macro_rules! deps {
    () => {
        DisplayTarget!();
        HirDatabase!();
    };
}

macro_rules! known_const_to_ast {
    () => {
        deps!();
        pub fn known_const_to_ast < 'db > (konst : Const < 'db > , db : & 'db dyn HirDatabase , display_target : DisplayTarget ,) -> Option < ConstArg > { Some (make :: expr_const_value (konst . display (db , display_target) . to_string () . as_str ())) }
    };
}

known_const_to_ast!()