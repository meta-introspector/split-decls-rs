macro_rules! deps {
    () => {
        Tag!();
        ObjectDetached!();
        Kind!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < 'repo > From < Tag < 'repo > > for ObjectDetached { fn from (mut v : Tag < 'repo >) -> Self { ObjectDetached { id : v . id , kind : gix_object :: Kind :: Tag , data : steal_from_freelist (& mut v . data) , } } }
    };
}

impl_164!();