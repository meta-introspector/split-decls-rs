macro_rules! deps {
    () => {
        LintStoreExpandImpl!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl LintStoreExpand for LintStoreExpandImpl < '_ > { fn pre_expansion_lint (& self , sess : & Session , features : & Features , registered_tools : & RegisteredTools , node_id : ast :: NodeId , attrs : & [ast :: Attribute] , items : & [Box < ast :: Item >] , name : Symbol ,) { pre_expansion_lint (sess , features , self . 0 , registered_tools , (node_id , attrs , items) , name) ; } }
    };
}

impl_41!()