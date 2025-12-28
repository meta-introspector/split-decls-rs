macro_rules! impl_420 {
    () => {
        impl < St , T > Future for NextIfEq < '_ , St , T > where St : Stream , T : ? Sized , St :: Item : PartialEq < T > , { type Output = Option < St :: Item > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . inner . poll (cx) } }
    };
}

impl_420!()