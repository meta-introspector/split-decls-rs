// Generated macro for impl_477 (impl)
macro_rules! Depcrate_constsimpl_477 {
() => {
// Module: crate::consts
// Provides: {"impl_477"}
// Dependencies: {}
impl < 'll > StaticCodegenMethods for CodegenCx < 'll , '_ > { # [doc = " Get a pointer to a global variable."] # [doc = ""] # [doc = " The pointer will always be in the default address space. If global variables default to a"] # [doc = " different address space, an addrspacecast is inserted."] fn static_addr_of (& self , cv : & 'll Value , align : Align , kind : Option < & str >) -> & 'll Value { let gv = self . static_addr_of_impl (cv , align , kind) ; self . const_pointercast (gv , self . type_ptr ()) } fn codegen_static (& mut self , def_id : DefId) { self . codegen_static_item (def_id) } }
};
}
