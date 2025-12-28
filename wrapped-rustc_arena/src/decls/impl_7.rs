macro_rules! deps {
    () => {
        TypedArena!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T > Default for TypedArena < T > { # [doc = " Creates a new `TypedArena`."] fn default () -> TypedArena < T > { TypedArena { ptr : Cell :: new (ptr :: null_mut ()) , end : Cell :: new (ptr :: null_mut ()) , chunks : Default :: default () , _own : PhantomData , } } }
    };
}

impl_7!()