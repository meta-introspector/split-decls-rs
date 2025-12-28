macro_rules! deps {
    () => {
        Attrs!();
        MockableStruct!();
        AttrFormatter!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl From < (Attrs , ItemTrait) > for MockableStruct { fn from ((attrs , item_trait) : (Attrs , ItemTrait)) -> MockableStruct { let trait_ = attrs . substitute_trait (& item_trait) ; let mut attrs = AttrFormatter :: new (& trait_ . attrs) . doc (true) . async_trait (true) . trait_variant (true) . must_use (false) . format () ; attrs . push (derive_debug ()) ; let vis = trait_ . vis . clone () ; let name = gen_mock_ident (& trait_ . ident) ; let generics = trait_ . generics . clone () ; let impls = vec ! [mockable_trait (trait_ , & name , & generics)] ; MockableStruct { attrs , consts : Vec :: new () , vis , name , generics , methods : Vec :: new () , impls } } }
    };
}

impl_104!()