// Generated macro for impl_905 (impl)
macro_rules! Depcrate_graph_impl_serializationimpl_905 {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"impl_905"}
// Dependencies: {}
impl < Ty > FromDeserialized for PhantomData < Ty > where Ty : EdgeType , { type Input = EdgeProperty ; fn from_deserialized < E2 > (input : Self :: Input) -> Result < Self , E2 > where E2 : Error , { if input . is_directed () != Ty :: is_directed () { Err (E2 :: custom (format_args ! ("graph edge property mismatch, \
                 expected {:?}, found {:?}" , EdgeProperty :: from (PhantomData ::< Ty >) , input))) } else { Ok (PhantomData) } } }
};
}
