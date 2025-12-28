macro_rules! insert_builtin {
    () => {
        macro_rules ! insert_builtin { ($ builtin : expr , $ name : ident , $ pattern : expr) => { $ builtin . push ((stringify ! ($ name) , generate_rule ! ($ name , $ pattern))) ; } ; }
    };
}

insert_builtin!()