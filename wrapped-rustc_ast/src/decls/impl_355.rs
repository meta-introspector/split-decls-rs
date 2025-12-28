macro_rules! deps {
    () => {
        MutVisitable!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl < V : MutVisitor , T : ? Sized > MutVisitable < V > for Box < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { (* * self) . visit_mut (visitor , extra) } }
    };
}

impl_355!();