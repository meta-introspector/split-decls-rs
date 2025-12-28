macro_rules! deps {
    () => {
        Size!();
        Float!();
        Primitive!();
        HasDataLayout!();
        AbiAlign!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl Primitive { pub fn size < C : HasDataLayout > (self , cx : & C) -> Size { use Primitive :: * ; let dl = cx . data_layout () ; match self { Int (i , _) => i . size () , Float (f) => f . size () , Pointer (a) => dl . pointer_size_in (a) , } } pub fn align < C : HasDataLayout > (self , cx : & C) -> AbiAlign { use Primitive :: * ; let dl = cx . data_layout () ; match self { Int (i , _) => i . align (dl) , Float (f) => f . align (dl) , Pointer (a) => dl . pointer_align_in (a) , } } }
    };
}

impl_92!()