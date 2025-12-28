macro_rules! deps {
    () => {
        Formatter!();
        Result!();
        Spans!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < 'e , E : core :: fmt :: Display > core :: fmt :: Display for Formatter < 'e , E > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let spans = Spans :: from_formatter (self) ; if self . pattern . contains ('\n') { let divider = repeat_char ('~' , 79) ; writeln ! (f , "regex parse error:") ? ; writeln ! (f , "{divider}") ? ; let notated = spans . notate () ; write ! (f , "{notated}") ? ; writeln ! (f , "{divider}") ? ; if ! spans . multi_line . is_empty () { let mut notes = vec ! [] ; for span in & spans . multi_line { notes . push (format ! ("on line {} (column {}) through line {} (column {})" , span . start . line , span . start . column , span . end . line , span . end . column - 1)) ; } writeln ! (f , "{}" , notes . join ("\n")) ? ; } write ! (f , "error: {}" , self . err) ? ; } else { writeln ! (f , "regex parse error:") ? ; let notated = Spans :: from_formatter (self) . notate () ; write ! (f , "{notated}") ? ; write ! (f , "error: {}" , self . err) ? ; } Ok (()) } }
    };
}

impl_138!()