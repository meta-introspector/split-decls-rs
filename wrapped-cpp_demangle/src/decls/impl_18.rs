macro_rules! deps {
    () => {
        ParseContextState!();
        ParseOptions!();
        ParseContext!();
        Result!();
        Error!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl ParseContext { # [doc = " Construct a new `ParseContext`."] pub fn new (options : ParseOptions) -> ParseContext { ParseContext { max_recursion : options . recursion_limit . map (| v | v . get ()) . unwrap_or (96) , state : Cell :: new (ParseContextState :: default ()) , } } # [doc = " Get the current recursion level for this context."] pub fn recursion_level (& self) -> u32 { self . state . get () . recursion_level } # [inline] fn enter_recursion (& self) -> error :: Result < () > { let mut state = self . state . get () ; let new_recursion_level = state . recursion_level + 1 ; if new_recursion_level >= self . max_recursion { log ! ("Hit too much recursion at level {}" , self . max_recursion) ; Err (error :: Error :: TooMuchRecursion) } else { state . recursion_level = new_recursion_level ; self . state . set (state) ; Ok (()) } } # [inline] fn exit_recursion (& self) { let mut state = self . state . get () ; debug_assert ! (state . recursion_level >= 1) ; state . recursion_level -= 1 ; self . state . set (state) ; } # [inline] fn in_conversion (& self) -> bool { self . state . get () . in_conversion } fn set_in_conversion (& self , in_conversion : bool) -> bool { let mut state = self . state . get () ; let previously_in_conversion = state . in_conversion ; state . in_conversion = in_conversion ; self . state . set (state) ; previously_in_conversion } }
    };
}

impl_18!()