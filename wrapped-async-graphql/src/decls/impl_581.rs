macro_rules! deps {
    () => {
        ExtensionContext!();
        Response!();
        NextSubscribe!();
    };
}

macro_rules! impl_581 {
    () => {
        deps!();
        impl NextSubscribe < '_ > { # [doc = " Call the [Extension::subscribe] function of next extension."] pub fn run < 's > (self , ctx : & ExtensionContext < '_ > , stream : BoxStream < 's , Response > ,) -> BoxStream < 's , Response > { if let Some ((first , next)) = self . chain . split_first () { first . subscribe (ctx , stream , NextSubscribe { chain : next }) } else { stream } } }
    };
}

impl_581!();