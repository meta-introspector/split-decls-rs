macro_rules! deps {
    () => {
        Result!();
        Sections!();
        Dwarf!();
        LineString!();
        Writer!();
    };
}

macro_rules! impl_738 {
    () => {
        deps!();
        impl Dwarf { # [doc = " Create a new `Dwarf` instance."] # [inline] pub fn new () -> Self { Self :: default () } # [doc = " Write the DWARF information to the given sections."] pub fn write < W : Writer > (& mut self , sections : & mut Sections < W >) -> Result < () > { self . units . write (sections , & mut self . line_strings , & mut self . strings) ? ; for line_program in & self . line_programs { line_program . write (& mut sections . debug_line , line_program . encoding () , & mut self . line_strings , & mut self . strings ,) ? ; } self . line_strings . write (& mut sections . debug_line_str) ? ; self . strings . write (& mut sections . debug_str) ? ; Ok (()) } # [doc = " Get a reference to the data for a line string."] pub fn get_line_string < 'a > (& 'a self , string : & 'a LineString) -> & 'a [u8] { string . get (& self . strings , & self . line_strings) } }
    };
}

impl_738!()