macro_rules! impl_17 {
    () => {
        impl < F1 , F2 > Future for Zip < F1 , F2 > where F1 : Future , F2 : Future , { type Output = (F1 :: Output , F2 :: Output) ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; if let Some (future) = this . future1 . as_mut () . as_pin_mut () { if let Poll :: Ready (out) = future . poll (cx) { * this . output1 = Some (out) ; this . future1 . set (None) ; } } if let Some (future) = this . future2 . as_mut () . as_pin_mut () { if let Poll :: Ready (out) = future . poll (cx) { * this . output2 = Some (out) ; this . future2 . set (None) ; } } take_zip_from_parts (this . output1 , this . output2) } }
    };
}

impl_17!()