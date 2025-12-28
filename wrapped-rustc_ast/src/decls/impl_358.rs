macro_rules! deps {
    () => {
        MutVisitable!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl < V : MutVisitor , T > MutVisitable < V > for [T] where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { for item in self { item . visit_mut (visitor , extra) ; } } }
    };
}

impl_358!()