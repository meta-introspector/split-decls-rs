macro_rules! deps {
    () => {
        MutVisitable!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl < V : MutVisitor , T > MutVisitable < V > for (T ,) where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { self . 0 . visit_mut (visitor , extra) ; } }
    };
}

impl_360!();