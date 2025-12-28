macro_rules! tuple_trait_inner {
    () => {
        macro_rules ! tuple_trait_inner (($ it : tt , $ self : expr , $ input : expr , () , $ head : ident $ ($ id : ident) +) => ({ let (i , o) = $ self .$ it . parse ($ input) ?; succ ! ($ it , tuple_trait_inner ! ($ self , i , (o) , $ ($ id) +)) }) ; ($ it : tt , $ self : expr , $ input : expr , ($ ($ parsed : tt) *) , $ head : ident $ ($ id : ident) +) => ({ let (i , o) = $ self .$ it . parse ($ input) ?; succ ! ($ it , tuple_trait_inner ! ($ self , i , ($ ($ parsed) * , o) , $ ($ id) +)) }) ; ($ it : tt , $ self : expr , $ input : expr , ($ ($ parsed : tt) *) , $ head : ident) => ({ let (i , o) = $ self .$ it . parse ($ input) ?; Ok ((i , ($ ($ parsed) * , o))) }) ;) ;
    };
}

tuple_trait_inner!()