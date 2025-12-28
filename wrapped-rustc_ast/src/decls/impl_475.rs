macro_rules! deps {
    () => {
        Visitable!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl < 'a , V : Visitor < 'a > , T1 , T2 > Visitable < 'a , V > for (T1 , T2) where T1 : Visitable < 'a , V , Extra = () > , T2 : Visitable < 'a , V , Extra = () > , { type Extra = () ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { try_visit ! (self . 0 . visit (visitor , extra)) ; try_visit ! (self . 1 . visit (visitor , extra)) ; V :: Result :: output () } }
    };
}

impl_475!()