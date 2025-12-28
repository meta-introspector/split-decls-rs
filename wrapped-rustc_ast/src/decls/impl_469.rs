macro_rules! deps {
    () => {
        Visitable!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl < 'a , V : Visitor < 'a > , T : ? Sized > Visitable < 'a , V > for Box < T > where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { (* * self) . visit (visitor , extra) } }
    };
}

impl_469!();