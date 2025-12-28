macro_rules! deps {
    () => {
        Visitable!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < 'a , V : Visitor < 'a > , T > Visitable < 'a , V > for Vec < T > where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { for item in self { try_visit ! (item . visit (visitor , extra)) ; } V :: Result :: output () } }
    };
}

impl_473!()