macro_rules! deps {
    () => {
        ArgScopeStack!();
        DemangleWrite!();
        Name!();
        DemangleContext!();
        Result!();
        UnresolvedName!();
        Demangle!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for UnresolvedName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { UnresolvedName :: Name (ref name) => name . demangle (ctx , scope) , UnresolvedName :: Global (ref name) => { write ! (ctx , "::") ? ; name . demangle (ctx , scope) } UnresolvedName :: Nested1 (ref ty , ref levels , ref name) => { ty . demangle (ctx , scope) ? ; write ! (ctx , "::") ? ; for lvl in & levels [..] { lvl . demangle (ctx , scope) ? ; write ! (ctx , "::") ? ; } name . demangle (ctx , scope) } UnresolvedName :: Nested2 (ref levels , ref name) => { for lvl in & levels [..] { lvl . demangle (ctx , scope) ? ; write ! (ctx , "::") ? ; } name . demangle (ctx , scope) } UnresolvedName :: GlobalNested2 (ref levels , ref name) => { write ! (ctx , "::") ? ; for lvl in & levels [..] { lvl . demangle (ctx , scope) ? ; write ! (ctx , "::") ? ; } name . demangle (ctx , scope) } } } }
    };
}

impl_248!()