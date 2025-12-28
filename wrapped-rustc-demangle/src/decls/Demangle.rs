macro_rules! deps {
    () => {
        DemangleStyle!();
    };
}

macro_rules! Demangle {
    () => {
        deps!();
        # [doc = " Representation of a demangled symbol name."] pub struct Demangle < 'a > { style : Option < DemangleStyle < 'a > > , original : & 'a str , suffix : & 'a str , }
    };
}

Demangle!();