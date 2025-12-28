macro_rules! deps {
    () => {
        AllocRef!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl < 'a , 'tcx , Prov : Provenance , Extra , Bytes : AllocBytes > AllocRef < 'a , 'tcx , Prov , Extra , Bytes > { # [doc = " `range` is relative to this allocation reference, not the base of the allocation."] pub fn read_scalar (& self , range : AllocRange , read_provenance : bool ,) -> InterpResult < 'tcx , Scalar < Prov > > { let range = self . range . subrange (range) ; self . alloc . read_scalar (& self . tcx , range , read_provenance) . map_err (| e | e . to_interp_error (self . alloc_id)) . into () } # [doc = " `range` is relative to this allocation reference, not the base of the allocation."] pub fn read_integer (& self , range : AllocRange) -> InterpResult < 'tcx , Scalar < Prov > > { self . read_scalar (range , false) } # [doc = " `offset` is relative to this allocation reference, not the base of the allocation."] pub fn read_pointer (& self , offset : Size) -> InterpResult < 'tcx , Scalar < Prov > > { self . read_scalar (alloc_range (offset , self . tcx . data_layout () . pointer_size ()) , true ,) } # [doc = " `range` is relative to this allocation reference, not the base of the allocation."] pub fn get_bytes_strip_provenance < 'b > (& 'b self) -> InterpResult < 'tcx , & 'a [u8] > { self . alloc . get_bytes_strip_provenance (& self . tcx , self . range) . map_err (| e | e . to_interp_error (self . alloc_id)) . into () } # [doc = " Returns whether the allocation has provenance anywhere in the range of the `AllocRef`."] pub fn has_provenance (& self) -> bool { ! self . alloc . provenance () . range_empty (self . range , & self . tcx) } }
    };
}

impl_259!()