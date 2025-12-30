// Generated macro for impl_36 (impl)
macro_rules! Depcrate_flatimpl_36 {
() => {
// Module: crate::flat
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'sval , S : Stream < 'sval > > FlatStream < 'sval , S > { pub fn new (stream : S) -> Self { FlatStream { buffered : None , state : State :: Any (Some (Any { stream , _marker : PhantomData , })) , } } pub fn finish (& mut self) -> Result < S :: Ok > { if let State :: Done (ref mut r) = self . state { r . take () . unwrap_or_else (| | Err (Error :: invalid_value ("incomplete stream"))) } else { Err (Error :: invalid_value ("incomplete stream")) } } }
};
}
