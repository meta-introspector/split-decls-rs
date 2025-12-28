macro_rules! MutVisitable {
    () => {
        pub (crate) trait MutVisitable < V : MutVisitor > { type Extra : Copy ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) ; }
    };
}

MutVisitable!()