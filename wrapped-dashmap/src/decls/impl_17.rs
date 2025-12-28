macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < K : Eq + Hash + Clone , V : Clone , S : Clone > Clone for DashMap < K , V , S > { fn clone (& self) -> Self { fn clone_rwlock < T : Clone > (lock : & CachePadded < RwLock < T > >) -> CachePadded < RwLock < T > > { CachePadded :: new (RwLock :: new (lock . read () . clone ())) } Self { shift : self . shift , shards : self . shards . iter () . map (clone_rwlock) . collect () , hasher : self . hasher . clone () , } } }
    };
}

impl_17!()