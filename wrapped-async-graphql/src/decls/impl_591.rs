macro_rules! deps {
    () => {
        NextResolve!();
        ExtensionContext!();
        ServerResult!();
        ResolveInfo!();
    };
}

macro_rules! impl_591 {
    () => {
        deps!();
        impl NextResolve < '_ > { # [doc = " Call the [Extension::resolve] function of next extension."] pub async fn run (self , ctx : & ExtensionContext < '_ > , info : ResolveInfo < '_ > ,) -> ServerResult < Option < Value > > { if let Some ((first , next)) = self . chain . split_first () { first . resolve (ctx , info , NextResolve { chain : next , resolve_fut : self . resolve_fut , } ,) . await } else { self . resolve_fut . await } } }
    };
}

impl_591!();