macro_rules! tuple_trait {
    () => {
        macro_rules ! tuple_trait (($ name1 : ident $ ty1 : ident , $ name2 : ident $ ty2 : ident , $ ($ name : ident $ ty : ident) ,*) => (tuple_trait ! (__impl $ name1 $ ty1 , $ name2 $ ty2 ; $ ($ name $ ty) ,*) ;) ; (__impl $ ($ name : ident $ ty : ident) ,+; $ name1 : ident $ ty1 : ident , $ ($ name2 : ident $ ty2 : ident) ,*) => (tuple_trait_impl ! ($ ($ name $ ty) ,+) ; tuple_trait ! (__impl $ ($ name $ ty) ,+ , $ name1 $ ty1 ; $ ($ name2 $ ty2) ,*) ;) ; (__impl $ ($ name : ident $ ty : ident) ,+; $ name1 : ident $ ty1 : ident) => (tuple_trait_impl ! ($ ($ name $ ty) ,+) ; tuple_trait_impl ! ($ ($ name $ ty) ,+, $ name1 $ ty1) ;) ;) ;
    };
}

tuple_trait!()