macro_rules! deps {
    () => {
        MutVisitable!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl < V : MutVisitor , T > MutVisitable < V > for Spanned < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { let Spanned { span , node } = self ; span . visit_mut (visitor , ()) ; node . visit_mut (visitor , extra) ; } }
    };
}

impl_357!()