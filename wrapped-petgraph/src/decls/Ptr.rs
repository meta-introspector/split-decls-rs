macro_rules! deps {
    () => {
        GraphMap!();
    };
}

macro_rules! Ptr {
    () => {
        deps!();
        # [doc = " A reference that is hashed and compared by its pointer value."] # [doc = ""] # [doc = " `Ptr` is used for certain configurations of `GraphMap`,"] # [doc = " in particular in the combination where the node type for"] # [doc = " `GraphMap` is something of type for example `Ptr(&Cell<T>)`,"] # [doc = " with the `Cell<T>` being `TypedArena` allocated."] pub struct Ptr < 'b , T : 'b > (pub & 'b T) ;
    };
}

Ptr!();