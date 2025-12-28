macro_rules! deps {
    () => {
        Visitable!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl < 'a , V : Visitor < 'a > , T > Visitable < 'a , V > for Option < T > where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { if let Some (this) = self { try_visit ! (this . visit (visitor , extra)) ; } V :: Result :: output () } }
    };
}

impl_470!();