macro_rules! alt_trait {
    () => {
        macro_rules ! alt_trait (($ first : ident $ second : ident $ ($ id : ident) +) => (alt_trait ! (__impl $ first $ second ; $ ($ id) +) ;) ; (__impl $ ($ current : ident) *; $ head : ident $ ($ id : ident) +) => (alt_trait_impl ! ($ ($ current) *) ; alt_trait ! (__impl $ ($ current) * $ head ; $ ($ id) +) ;) ; (__impl $ ($ current : ident) *; $ head : ident) => (alt_trait_impl ! ($ ($ current) *) ; alt_trait_impl ! ($ ($ current) * $ head) ;) ;) ;
    };
}

alt_trait!()