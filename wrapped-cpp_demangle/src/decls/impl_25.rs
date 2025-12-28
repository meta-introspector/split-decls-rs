macro_rules! deps {
    () => {
        SourceName!();
        DemangleContext!();
        DemangleWrite!();
        DemangleAsLeaf!();
        LeafName!();
        Result!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsLeaf < 'subs , W > for LeafName < 'subs > where W : 'subs + DemangleWrite , { fn demangle_as_leaf < 'me , 'ctx > (& 'me self , ctx : & 'ctx mut DemangleContext < 'subs , W > ,) -> fmt :: Result { match * self { LeafName :: SourceName (sn) => sn . demangle (ctx , None) , LeafName :: Closure (c) => c . demangle (ctx , None) , LeafName :: WellKnownComponent (wkc) => wkc . demangle_as_leaf (ctx) , LeafName :: UnnamedType (utn) => utn . demangle_as_leaf (ctx) , } } }
    };
}

impl_25!()