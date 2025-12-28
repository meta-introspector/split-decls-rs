macro_rules! deps {
    () => {
        NodeReferences!();
        IdStorage!();
    };
}

macro_rules! impl_995 {
    () => {
        deps!();
        impl < 'a , N : 'a , Ix , S : BuildHasher > NodeReferences < 'a , N , Ix , S > { fn new (nodes : & 'a IdStorage < N , S >) -> Self { NodeReferences { nodes , iter : nodes . iter_ids () , ix : PhantomData , } } }
    };
}

impl_995!();