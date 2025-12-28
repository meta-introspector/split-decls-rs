macro_rules! impl_59 {
    () => {
        impl < F1 , F2 > Future for Either < F1 , F2 > where F1 : Future , F2 : Future < Output = F1 :: Output > , { type Output = F1 :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () { EitherProj :: Left { fut } => fut . poll (cx) , EitherProj :: Right { fut } => fut . poll (cx) , } } }
    };
}

impl_59!()