macro_rules! deps {
    () => {
        MutVisitable!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        impl < V : MutVisitor , T > MutVisitable < V > for Option < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { if let Some (this) = self { this . visit_mut (visitor , extra) } } }
    };
}

impl_356!();