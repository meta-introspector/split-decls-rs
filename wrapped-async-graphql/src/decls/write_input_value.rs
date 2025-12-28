macro_rules! deps {
    () => {
        MetaInputValue!();
    };
}

macro_rules! write_input_value {
    () => {
        deps!();
        fn write_input_value (sdl : & mut String , input_value : & MetaInputValue) { if let Some (default_value) = & input_value . default_value { _ = write ! (sdl , "{}: {} = {}" , input_value . name , input_value . ty , default_value) ; } else { _ = write ! (sdl , "{}: {}" , input_value . name , input_value . ty) ; } write_deprecated (sdl , & input_value . deprecation) ; }
    };
}

write_input_value!()