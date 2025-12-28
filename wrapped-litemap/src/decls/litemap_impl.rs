macro_rules! litemap_impl {
    () => {
        macro_rules ! litemap_impl (($ cfg : meta , $ store : ident $ (=$ defaultty : ty) ?) => { # [doc = " A simple \"flat\" map based on a sorted vector"] # [doc = ""] # [doc = " See the [module level documentation][super] for why one should use this."] # [doc = ""] # [doc = " The API is roughly similar to that of [`std::collections::BTreeMap`]."] # [derive (Clone , Debug , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "yoke" , derive (yoke :: Yokeable))] # [cfg ($ cfg)] pub struct LiteMap < K : ? Sized , V : ? Sized , $ store $ (= $ defaultty) ?> { pub (crate) values : $ store , pub (crate) _key_type : PhantomData < K >, pub (crate) _value_type : PhantomData < V >, } } ;) ;
    };
}

litemap_impl!();