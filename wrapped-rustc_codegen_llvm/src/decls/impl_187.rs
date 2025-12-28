macro_rules! deps {
    () => {
        SCx!();
        GenericCx!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { pub (crate) fn const_array (& self , ty : & 'll Type , elts : & [& 'll Value]) -> & 'll Value { let len = u64 :: try_from (elts . len ()) . expect ("LLVMConstArray2 elements len overflow") ; unsafe { llvm :: LLVMConstArray2 (ty , elts . as_ptr () , len) } } pub (crate) fn const_bytes (& self , bytes : & [u8]) -> & 'll Value { bytes_in_context (self . llcx () , bytes) } pub (crate) fn const_get_elt (& self , v : & 'll Value , idx : u64) -> & 'll Value { unsafe { let idx = c_uint :: try_from (idx) . expect ("LLVMGetAggregateElement index overflow") ; let r = llvm :: LLVMGetAggregateElement (v , idx) . unwrap () ; debug ! ("const_get_elt(v={:?}, idx={}, r={:?})" , v , idx , r) ; r } } pub (crate) fn const_null (& self , t : & 'll Type) -> & 'll Value { unsafe { llvm :: LLVMConstNull (t) } } }
    };
}

impl_187!();