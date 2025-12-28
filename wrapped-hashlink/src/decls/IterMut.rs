macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        pub struct IterMut < 'a , K , V > { head : Option < NonNull < Node < K , V > > > , tail : Option < NonNull < Node < K , V > > > , remaining : usize , marker : PhantomData < (& 'a K , & 'a mut V) > , }
    };
}

IterMut!()