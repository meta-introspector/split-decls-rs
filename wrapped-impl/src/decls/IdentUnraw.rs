macro_rules! IdentUnraw {
    () => {
        # [derive (Clone)] # [repr (transparent)] pub struct IdentUnraw (Ident) ;
    };
}

IdentUnraw!();