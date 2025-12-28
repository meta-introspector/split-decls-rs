macro_rules! deps {
    () => {
        VirtualIndex!();
        BuilderMethods!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl < 'a , 'tcx > VirtualIndex { pub (crate) fn from_index (index : usize) -> Self { VirtualIndex (index as u64) } fn get_fn_inner < Bx : BuilderMethods < 'a , 'tcx > > (self , bx : & mut Bx , llvtable : Bx :: Value , ty : Ty < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , nonnull : bool ,) -> Bx :: Value { debug ! ("get_fn({llvtable:?}, {ty:?}, {self:?})") ; let llty = bx . fn_ptr_backend_type (fn_abi) ; let ptr_size = bx . data_layout () . pointer_size () ; let vtable_byte_offset = self . 0 * ptr_size . bytes () ; load_vtable (bx , llvtable , llty , vtable_byte_offset , ty , nonnull) } pub (crate) fn get_optional_fn < Bx : BuilderMethods < 'a , 'tcx > > (self , bx : & mut Bx , llvtable : Bx :: Value , ty : Ty < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > ,) -> Bx :: Value { self . get_fn_inner (bx , llvtable , ty , fn_abi , false) } pub (crate) fn get_fn < Bx : BuilderMethods < 'a , 'tcx > > (self , bx : & mut Bx , llvtable : Bx :: Value , ty : Ty < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > ,) -> Bx :: Value { self . get_fn_inner (bx , llvtable , ty , fn_abi , true) } pub (crate) fn get_usize < Bx : BuilderMethods < 'a , 'tcx > > (self , bx : & mut Bx , llvtable : Bx :: Value , ty : Ty < 'tcx > ,) -> Bx :: Value { debug ! ("get_int({:?}, {:?})" , llvtable , self) ; let llty = bx . type_isize () ; let ptr_size = bx . data_layout () . pointer_size () ; let vtable_byte_offset = self . 0 * ptr_size . bytes () ; load_vtable (bx , llvtable , llty , vtable_byte_offset , ty , false) } }
    };
}

impl_438!();