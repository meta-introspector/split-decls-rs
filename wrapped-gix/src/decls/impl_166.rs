macro_rules! deps {
    () => {
        Kind!();
        ObjectDetached!();
        Tree!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < 'repo > From < Tree < 'repo > > for ObjectDetached { fn from (mut v : Tree < 'repo >) -> Self { ObjectDetached { id : v . id , kind : gix_object :: Kind :: Tree , data : steal_from_freelist (& mut v . data) , } } }
    };
}

impl_166!();