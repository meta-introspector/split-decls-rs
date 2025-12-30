// Generated macro for TreeServer (struct)
macro_rules! Depcrate_objectpathTreeServer {
() => {
// Module: crate::objectpath
// Provides: {"TreeServer"}
// Dependencies: {}
# [doc = " An iterator adapter that handles incoming method calls."] # [doc = ""] # [doc = " Method calls that match an object path in the tree are handled and consumed by this"] # [doc = " iterator. Other messages are passed through."] pub struct TreeServer < 'a , I , M : MethodType < D > + 'a , D : DataType + 'a > { iter : I , conn : & 'a Connection , tree : & 'a Tree < M , D > , }
};
}
