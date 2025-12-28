macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! param_idx {
    () => {
        deps!();
        # [doc = " Return an index of a parameter in the generic type parameter list by it's id."] pub fn param_idx (db : & dyn HirDatabase , id : TypeOrConstParamId) -> Option < usize > { generics :: generics (db , id . parent) . type_or_const_param_idx (id) }
    };
}

param_idx!()