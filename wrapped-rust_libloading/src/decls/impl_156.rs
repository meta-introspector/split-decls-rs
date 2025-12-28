macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < 'lib , T > Symbol < 'lib , Option < T > > { # [doc = " Lift Option out of the symbol."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use ::libloading::{Library, Symbol};"] # [doc = " unsafe {"] # [doc = "     let lib = Library::new(\"/path/to/awesome.module\").unwrap();"] # [doc = "     let symbol: Symbol<Option<*mut u32>> = lib.get(b\"symbol\\0\").unwrap();"] # [doc = "     let symbol: Symbol<*mut u32> = symbol.lift_option().expect(\"static is not null\");"] # [doc = " }"] # [doc = " ```"] pub fn lift_option (self) -> Option < Symbol < 'lib , T > > { self . inner . lift_option () . map (| is | Symbol { inner : is , pd : marker :: PhantomData , }) } }
    };
}

impl_156!()