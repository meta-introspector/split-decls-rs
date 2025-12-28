macro_rules! deps {
    () => {
        MutVisitable!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl < V : MutVisitor , T1 , T2 > MutVisitable < V > for (T1 , T2) where T1 : MutVisitable < V , Extra = () > , T2 : MutVisitable < V , Extra = () > , { type Extra = () ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { self . 0 . visit_mut (visitor , extra) ; self . 1 . visit_mut (visitor , extra) ; } }
    };
}

impl_361!();