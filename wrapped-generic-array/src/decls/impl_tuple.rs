macro_rules! deps {
    () => {
        GenericArray!();
    };
}

macro_rules! impl_tuple {
    () => {
        deps!();
        macro_rules ! impl_tuple { (@ T $ t : ident) => { T } ; ($ ($ len : ty => ($ ($ t : ident ,) *) ;) *) => { $ (impl < T > From < ($ (impl_tuple ! (@ T $ t) ,) *) > for GenericArray < T , $ len > { # [inline] # [allow (non_snake_case)] fn from (tuple : ($ (impl_tuple ! (@ T $ t) ,) *)) -> Self { let ($ ($ t ,) *) = tuple ; GenericArray :: from_array ([$ ($ t ,) *]) } } impl < T > From < GenericArray < T , $ len >> for ($ (impl_tuple ! (@ T $ t) ,) *) { # [inline] # [allow (non_snake_case)] fn from (array : GenericArray < T , $ len >) -> Self { let [$ ($ t) ,*] = array . into_array () ; ($ ($ t ,) *) } }) * } ; }
    };
}

impl_tuple!()