macro_rules! deps {
    () => {
        Selection!();
        Positioned!();
        FragmentSpread!();
        Field!();
        InlineFragment!();
        Directive!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Selection { # [doc = " Get a reference to the directives of the selection."] # [must_use] pub fn directives (& self) -> & Vec < Positioned < Directive > > { match self { Self :: Field (field) => & field . node . directives , Self :: FragmentSpread (spread) => & spread . node . directives , Self :: InlineFragment (fragment) => & fragment . node . directives , } } # [doc = " Get a mutable reference to the directives of the selection."] # [must_use] pub fn directives_mut (& mut self) -> & mut Vec < Positioned < Directive > > { match self { Self :: Field (field) => & mut field . node . directives , Self :: FragmentSpread (spread) => & mut spread . node . directives , Self :: InlineFragment (fragment) => & mut fragment . node . directives , } } }
    };
}

impl_13!();