macro_rules! deps {
    () => {
        EncoderWriter!();
        Engine!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'e , E : Engine , W : io :: Write > fmt :: Debug for EncoderWriter < 'e , E , W > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "extra_input: {:?} extra_input_occupied_len:{:?} output[..5]: {:?} output_occupied_len: {:?}" , self . extra_input , self . extra_input_occupied_len , & self . output [0 .. 5] , self . output_occupied_len) } }
    };
}

impl_44!()