macro_rules! demutify {
    () => {
        # [doc = " Remove any mutability qualifiers from a method's argument list"] fn demutify (inputs : & mut Punctuated < FnArg , token :: Comma >) { for arg in inputs . iter_mut () { match arg { FnArg :: Receiver (r) => if r . reference . is_none () { r . mutability = None } , FnArg :: Typed (pt) => demutify_arg (pt) , } } }
    };
}

demutify!();