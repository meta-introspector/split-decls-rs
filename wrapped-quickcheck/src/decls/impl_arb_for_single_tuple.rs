macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_arb_for_single_tuple {
    () => {
        deps!();
        macro_rules ! impl_arb_for_single_tuple { ($ (($ type_param : ident , $ tuple_index : tt) ,) *) => { impl <$ ($ type_param) ,*> Arbitrary for ($ ($ type_param ,) *) where $ ($ type_param : Arbitrary ,) * { fn arbitrary (g : & mut Gen) -> ($ ($ type_param ,) *) { ($ ($ type_param :: arbitrary (g) ,) *) } fn shrink (& self) -> Box < dyn Iterator < Item = ($ ($ type_param ,) *) >> { let iter = :: std :: iter :: empty () ; $ (let cloned = self . clone () ; let iter = iter . chain (self .$ tuple_index . shrink () . map (move | shr_value | { let mut result = cloned . clone () ; result .$ tuple_index = shr_value ; result })) ;) * Box :: new (iter) } } } ; }
    };
}

impl_arb_for_single_tuple!()