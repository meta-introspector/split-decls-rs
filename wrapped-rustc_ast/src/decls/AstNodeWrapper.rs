macro_rules! AstNodeWrapper {
    () => {
        # [doc = " A newtype around an AST node that implements the traits above if the node implements them."] # [repr (transparent)] pub struct AstNodeWrapper < Wrapped , Tag > { pub wrapped : Wrapped , pub tag : PhantomData < Tag > , }
    };
}

AstNodeWrapper!()