macro_rules! deps {
    () => {
        ResponseBuilderExt!();
        ResponseUrl!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl ResponseBuilderExt for http :: response :: Builder { fn url (self , url : Url) -> Self { self . extension (ResponseUrl (url)) } }
    };
}

impl_51!();