macro_rules! deps {
    () => {
        IndexValue!();
        Index!();
        IndexAllocator!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl IndexAllocator { pub (crate) fn new () -> Self { IndexAllocator { next_const_index : 0 , explicit : false , } } pub (crate) fn const_index_of (explicit : IndexValue) -> Index { match explicit { IndexValue :: Const (explicit) => Index :: Explicit (quote ! (# explicit)) , IndexValue :: Ident (explicit) => Index :: Explicit (explicit) , } } pub (crate) fn next_const_index (& mut self , explicit : Option < IndexValue >) -> Index { if let Some (explicit) = explicit { self . explicit = true ; let index = match explicit { IndexValue :: Const (explicit) => explicit , _ => self . next_const_index , } ; self . next_const_index = index + 1 ; Self :: const_index_of (explicit) } else { let index = self . next_const_index ; self . next_const_index += 1 ; if self . explicit { Index :: Explicit (quote ! (# index)) } else { Index :: Implicit (quote ! (# index)) } } } pub (crate) fn next_computed_index (& mut self , ident : & syn :: Ident , explicit : Option < IndexValue > ,) -> Index { match self . next_const_index (explicit) { Index :: Implicit (_) => Index :: Implicit (quote ! ({ let index = # ident ; # ident += 1 ; index })) , Index :: Explicit (index) => Index :: Explicit (index) , } } }
    };
}

impl_59!()