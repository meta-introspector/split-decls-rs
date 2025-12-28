macro_rules! deps {
    () => {
        LayoutError!();
        Layout!();
        HirDatabase!();
        TraitEnvironment!();
    };
}

macro_rules! layout_of_ty_cycle_result {
    () => {
        deps!();
        pub (crate) fn layout_of_ty_cycle_result < 'db > (_db : & dyn HirDatabase , _salsa_id : salsa :: Id , _ty : Ty < 'db > , _trait_env : Arc < TraitEnvironment < 'db > > ,) -> Result < Arc < Layout > , LayoutError > { Err (LayoutError :: RecursiveTypeWithoutIndirection) }
    };
}

layout_of_ty_cycle_result!()