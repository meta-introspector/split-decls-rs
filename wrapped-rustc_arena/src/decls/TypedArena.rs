macro_rules! deps {
    () => {
        ArenaChunk!();
    };
}

macro_rules! TypedArena {
    () => {
        deps!();
        # [doc = " An arena that can hold objects of only one type."] pub struct TypedArena < T > { # [doc = " A pointer to the next object to be allocated."] ptr : Cell < * mut T > , # [doc = " A pointer to the end of the allocated area. When this pointer is"] # [doc = " reached, a new chunk is allocated."] end : Cell < * mut T > , # [doc = " A vector of arena chunks."] chunks : RefCell < Vec < ArenaChunk < T > > > , # [doc = " Marker indicating that dropping the arena causes its owned"] # [doc = " instances of `T` to be dropped."] _own : PhantomData < T > , }
    };
}

TypedArena!();