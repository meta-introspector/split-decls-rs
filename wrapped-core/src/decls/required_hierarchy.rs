macro_rules! deps {
    () => {
        CanInto!();
    };
}

macro_rules! required_hierarchy {
    () => {
        deps!();
        # [doc (hidden)] # [macro_export] macro_rules ! required_hierarchy { ($ child : ident , $ parent : ty) => { impl :: windows_core :: imp :: CanInto <$ parent > for $ child { const QUERY : bool = true ; } } ; ($ child : ident , $ first : ty , $ ($ rest : ty) ,+) => { $ crate :: imp :: required_hierarchy ! ($ child , $ first) ; $ crate :: imp :: required_hierarchy ! ($ child , $ ($ rest) ,+) ; } ; }
    };
}

required_hierarchy!()