macro_rules! deps {
    () => {
        CanInto!();
    };
}

macro_rules! interface_hierarchy {
    () => {
        deps!();
        # [doc (hidden)] # [macro_export] macro_rules ! interface_hierarchy { ($ child : ident , $ parent : ty) => { impl :: windows_core :: imp :: CanInto <$ parent > for $ child { } impl :: core :: convert :: From <&$ child > for &$ parent { fn from (value : &$ child) -> Self { unsafe { :: core :: mem :: transmute (value) } } } impl :: core :: convert :: From <$ child > for $ parent { fn from (value : $ child) -> Self { unsafe { :: core :: mem :: transmute (value) } } } } ; ($ child : ident , $ first : ty , $ ($ rest : ty) ,+) => { $ crate :: imp :: interface_hierarchy ! ($ child , $ first) ; $ crate :: imp :: interface_hierarchy ! ($ child , $ ($ rest) ,+) ; } ; }
    };
}

interface_hierarchy!()