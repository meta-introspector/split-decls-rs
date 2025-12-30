// Generated macro for Index (struct)
macro_rules! Depcrate_ex_dataIndex {
() => {
// Module: crate::ex_data
// Provides: {"Index"}
// Dependencies: {}
# [doc = " A slot in a type's \"extra data\" structure."] # [doc = ""] # [doc = " It is parameterized over the type containing the extra data as well as the"] # [doc = " type of the data in the slot."] pub struct Index < T , U > (c_int , PhantomData < (T , U) >) ;
};
}
