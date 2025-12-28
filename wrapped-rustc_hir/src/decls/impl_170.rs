macro_rules! deps {
    () => {
        OwnerNodes!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl fmt :: Debug for OwnerNodes < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OwnerNodes") . field ("node" , & self . nodes [ItemLocalId :: ZERO]) . field ("parents" , & fmt :: from_fn (| f | { f . debug_list () . entries (self . nodes . iter_enumerated () . map (| (id , parented_node) | { fmt :: from_fn (move | f | write ! (f , "({id:?}, {:?})" , parented_node . parent)) })) . finish () }) ,) . field ("bodies" , & self . bodies) . field ("opt_hash_including_bodies" , & self . opt_hash_including_bodies) . finish () } }
    };
}

impl_170!();