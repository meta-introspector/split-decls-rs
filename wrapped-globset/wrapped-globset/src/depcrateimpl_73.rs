// Generated macro for impl_73 (impl)
macro_rules! Depcrateimpl_73 {
() => {
// Module: crate
// Provides: {"impl_73"}
// Dependencies: {}
impl std :: fmt :: Display for ErrorKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match * self { ErrorKind :: InvalidRecursive | ErrorKind :: UnclosedClass | ErrorKind :: UnopenedAlternates | ErrorKind :: UnclosedAlternates | ErrorKind :: NestedAlternates | ErrorKind :: DanglingEscape | ErrorKind :: Regex (_) => write ! (f , "{}" , self . description ()) , ErrorKind :: InvalidRange (s , e) => { write ! (f , "invalid range; '{}' > '{}'" , s , e) } } } }
};
}
