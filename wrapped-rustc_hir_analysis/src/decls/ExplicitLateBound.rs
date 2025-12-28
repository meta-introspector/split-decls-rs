macro_rules! ExplicitLateBound {
    () => {
        # [doc = " New-typed boolean indicating whether explicit late-bound lifetimes"] # [doc = " are present in a set of generic arguments."] # [doc = ""] # [doc = " For example if we have some method `fn f<'a>(&'a self)` implemented"] # [doc = " for some type `T`, although `f` is generic in the lifetime `'a`, `'a`"] # [doc = " is late-bound so should not be provided explicitly. Thus, if `f` is"] # [doc = " instantiated with some generic arguments providing `'a` explicitly,"] # [doc = " we taint those arguments with `ExplicitLateBound::Yes` so that we"] # [doc = " can provide an appropriate diagnostic later."] # [derive (Copy , Clone , PartialEq , Debug)] pub enum ExplicitLateBound { Yes , No , }
    };
}

ExplicitLateBound!()