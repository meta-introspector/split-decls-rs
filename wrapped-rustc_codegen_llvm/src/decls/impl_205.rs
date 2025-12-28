macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < 'll > StaticCodegenMethods for CodegenCx < 'll , '_ > { # [doc = " Get a pointer to a global variable."] # [doc = ""] # [doc = " The pointer will always be in the default address space. If global variables default to a"] # [doc = " different address space, an addrspacecast is inserted."] fn static_addr_of (& self , cv : & 'll Value , align : Align , kind : Option < & str >) -> & 'll Value { let gv = self . static_addr_of_impl (cv , align , kind) ; self . const_pointercast (gv , self . type_ptr ()) } fn codegen_static (& mut self , def_id : DefId) { self . codegen_static_item (def_id) } }
    };
}

impl_205!();