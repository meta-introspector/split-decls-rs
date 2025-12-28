macro_rules! deps {
    () => {
        Visitable!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl < 'a , V : Visitor < 'a > , T > Visitable < 'a , V > for (T ,) where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { self . 0 . visit (visitor , extra) } }
    };
}

impl_474!();