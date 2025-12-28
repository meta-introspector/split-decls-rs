macro_rules! deps {
    () => {
        ImplTrait!();
    };
}

macro_rules! ImplTraits {
    () => {
        deps!();
        # [derive (PartialEq , Eq , Debug , Hash)] pub struct ImplTraits < 'db > { pub (crate) impl_traits : Arena < ImplTrait < 'db > > , }
    };
}

ImplTraits!();