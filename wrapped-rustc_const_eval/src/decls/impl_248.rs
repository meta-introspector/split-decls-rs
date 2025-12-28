macro_rules! deps {
    () => {
        FnVal!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < 'tcx , Other > FnVal < 'tcx , Other > { pub fn as_instance (self) -> InterpResult < 'tcx , Instance < 'tcx > > { match self { FnVal :: Instance (instance) => interp_ok (instance) , FnVal :: Other (_) => { throw_unsup_format ! ("'foreign' function pointers are not supported in this context") } } } }
    };
}

impl_248!();