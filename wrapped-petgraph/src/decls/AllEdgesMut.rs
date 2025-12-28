macro_rules! deps {
    () => {
        NodeTrait!();
    };
}

macro_rules! AllEdgesMut {
    () => {
        deps!();
        pub struct AllEdgesMut < 'a , N , E : 'a , Ty > where N : 'a + NodeTrait , { inner : IndexMapIterMut < 'a , (N , N) , E > , ty : PhantomData < Ty > , }
    };
}

AllEdgesMut!()