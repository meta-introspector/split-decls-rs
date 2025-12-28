macro_rules! Qualified {
    () => {
        # [derive (Debug)] pub (crate) enum Qualified < 'db > { No , With { path : ast :: Path , resolution : Option < PathResolution > , # [doc = " How many `super` segments are present in the path"] # [doc = ""] # [doc = " This would be None, if path is not solely made of"] # [doc = " `super` segments, e.g."] # [doc = ""] # [doc = " ```ignore"] # [doc = " use super::foo;"] # [doc = " ```"] # [doc = ""] # [doc = " Otherwise it should be Some(count of `super`)"] super_chain_len : Option < usize > , } , # [doc = " <_>::"] TypeAnchor { ty : Option < hir :: Type < 'db > > , trait_ : Option < hir :: Trait > , } , # [doc = " Whether the path is an absolute path"] Absolute , }
    };
}

Qualified!();