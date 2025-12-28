macro_rules! deps {
    () => {
        Formatter!();
        WriteStyle!();
        Writer!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Formatter { pub (crate) fn new (writer : & Writer) -> Self { Formatter { buf : Rc :: new (RefCell :: new (writer . buffer ())) , write_style : writer . write_style () , } } pub (crate) fn write_style (& self) -> WriteStyle { self . write_style } pub (crate) fn print (& self , writer : & Writer) -> io :: Result < () > { writer . print (& self . buf . borrow ()) } pub (crate) fn clear (& mut self) { self . buf . borrow_mut () . clear () ; } }
    };
}

impl_62!()