macro_rules! deselfify_args {
    () => {
        # [doc = " Change any `Self` in a method's arguments' types with `actual`."] # [doc = " `generics` is the Generics field of the parent struct."] fn deselfify_args (args : & mut Punctuated < FnArg , Token ! [,] > , actual : & Ident , generics : & Generics) { for arg in args . iter_mut () { match arg { FnArg :: Receiver (r) => { if r . colon_token . is_some () { deselfify (r . ty . as_mut () , actual , generics) } } , FnArg :: Typed (pt) => deselfify (pt . ty . as_mut () , actual , generics) } } }
    };
}

deselfify_args!();