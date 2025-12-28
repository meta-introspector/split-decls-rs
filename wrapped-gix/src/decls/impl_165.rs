macro_rules! deps {
    () => {
        Blob!();
        ObjectDetached!();
        Kind!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < 'repo > From < Blob < 'repo > > for ObjectDetached { fn from (mut v : Blob < 'repo >) -> Self { ObjectDetached { id : v . id , kind : gix_object :: Kind :: Blob , data : steal_from_freelist (& mut v . data) , } } }
    };
}

impl_165!()