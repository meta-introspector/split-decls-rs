macro_rules! deps {
    () => {
        CodegenCx!();
        LlvmType!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl LlvmType for Reg { fn llvm_type < 'll > (& self , cx : & CodegenCx < 'll , '_ >) -> & 'll Type { match self . kind { RegKind :: Integer => cx . type_ix (self . size . bits ()) , RegKind :: Float => match self . size . bits () { 16 => cx . type_f16 () , 32 => cx . type_f32 () , 64 => cx . type_f64 () , 128 => cx . type_f128 () , _ => bug ! ("unsupported float: {:?}" , self) , } , RegKind :: Vector => cx . type_vector (cx . type_i8 () , self . size . bytes ()) , } } }
    };
}

impl_6!()