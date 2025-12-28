macro_rules! Directive {
    () => {
        # [derive (Debug , Clone)] pub (crate) struct Directive { pub (crate) name : Option < String > , pub (crate) level : LevelFilter , }
    };
}

Directive!()