macro_rules! deps {
    () => {
        MultiUnzip!();
    };
}

macro_rules! impl_unzip_iter {
    () => {
        deps!();
        macro_rules ! impl_unzip_iter { ($ ($ T : ident => $ FromT : ident) ,*) => (# [allow (non_snake_case)] impl < IT : Iterator < Item = ($ ($ T ,) *) >, $ ($ T , $ FromT : Default + Extend <$ T >) ,* > MultiUnzip < ($ ($ FromT ,) *) > for IT { fn multiunzip (self) -> ($ ($ FromT ,) *) { let mut res = ($ ($ FromT :: default () ,) *) ; let ($ ($ FromT ,) *) = & mut res ; self . fold (() , | () , ($ ($ T ,) *) | { $ ($ FromT . extend (std :: iter :: once ($ T)) ;) * }) ; res } }) ; }
    };
}

impl_unzip_iter!();