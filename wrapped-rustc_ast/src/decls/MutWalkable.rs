macro_rules! MutWalkable {
    () => {
        pub trait MutWalkable < V : MutVisitor > { fn walk_mut (& mut self , visitor : & mut V) ; }
    };
}

MutWalkable!();