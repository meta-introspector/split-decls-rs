macro_rules! UnionItem {
    () => {
        # [derive (FromVariant)] # [darling (attributes (graphql))] pub struct UnionItem { pub ident : Ident , pub fields : Fields < syn :: Type > , # [darling (default)] pub flatten : bool , }
    };
}

UnionItem!()