macro_rules! deps {
    () => {
        Number!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl ToTokens for Number { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Number :: F64 (n) => tokens . extend (quote ! (# n as f64)) , Number :: I64 (n) => tokens . extend (quote ! (# n as i64)) , } } }
    };
}

impl_129!()