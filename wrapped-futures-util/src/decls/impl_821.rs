macro_rules! impl_821 {
    () => {
        impl < T > Future for OrderWrapper < T > where T : Future , { type Output = OrderWrapper < T :: Output > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let index = self . index ; self . project () . data . poll (cx) . map (| output | OrderWrapper { data : output , index }) } }
    };
}

impl_821!()