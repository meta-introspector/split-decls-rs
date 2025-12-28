macro_rules! deps {
    () => {
        MsvcBasicName!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl MsvcBasicName for ty :: FloatTy { fn msvc_basic_name (self) -> & 'static str { match self { ty :: FloatTy :: F16 => { bug ! ("`f16` should have been handled in `build_basic_type_di_node`") } ty :: FloatTy :: F32 => "float" , ty :: FloatTy :: F64 => "double" , ty :: FloatTy :: F128 => "fp128" , } } }
    };
}

impl_321!()