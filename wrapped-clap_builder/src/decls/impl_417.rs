macro_rules! deps {
    () => {
        Styles!();
        Message!();
        Command!();
        StyledStr!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl Message { fn format (& mut self , cmd : & Command , usage : Option < StyledStr >) { match self { Message :: Raw (s) => { let mut message = String :: new () ; std :: mem :: swap (s , & mut message) ; let styled = format :: format_error_message (& message , cmd . get_styles () , Some (cmd) , usage . as_ref () ,) ; * self = Self :: Formatted (styled) ; } Message :: Formatted (_) => { } } } fn formatted (& self , styles : & Styles) -> Cow < '_ , StyledStr > { match self { Message :: Raw (s) => { let styled = format :: format_error_message (s , styles , None , None) ; Cow :: Owned (styled) } Message :: Formatted (s) => Cow :: Borrowed (s) , } } }
    };
}

impl_417!()