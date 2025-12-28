macro_rules! deps {
    () => {
        FieldlessVariantsStrategy!();
        CombineSubstructureFunc!();
        Ty!();
        Bounds!();
    };
}

macro_rules! MethodDef {
    () => {
        deps!();
        pub (crate) struct MethodDef < 'a > { # [doc = " name of the method"] pub name : Symbol , # [doc = " List of generics, e.g., `R: rand::Rng`"] pub generics : Bounds , # [doc = " Is there is a `&self` argument? If not, it is a static function."] pub explicit_self : bool , # [doc = " Arguments other than the self argument."] pub nonself_args : Vec < (Ty , Symbol) > , # [doc = " Returns type"] pub ret_ty : Ty , pub attributes : ast :: AttrVec , pub fieldless_variants_strategy : FieldlessVariantsStrategy , pub combine_substructure : RefCell < CombineSubstructureFunc < 'a > > , }
    };
}

MethodDef!()