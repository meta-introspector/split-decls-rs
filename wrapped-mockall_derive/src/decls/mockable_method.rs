macro_rules! mockable_method {
    () => {
        # [doc = " Performs transformations on the method to make it mockable"] fn mockable_method (meth : & mut ImplItemFn , name : & Ident , generics : & Generics) { demutify (& mut meth . sig . inputs) ; deselfify_args (& mut meth . sig . inputs , name , generics) ; add_lifetime_parameters (& mut meth . sig) ; deimplify (& mut meth . sig . output) ; dewhereselfify (& mut meth . sig . generics) ; if let ReturnType :: Type (_ , ty) = & mut meth . sig . output { deselfify (ty , name , generics) ; deanonymize (ty) ; } sanity_check_sig (& meth . sig) ; }
    };
}

mockable_method!();