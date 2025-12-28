macro_rules! Resolvability {
    () => {
        # [derive (Debug , Clone , Default)] pub enum Resolvability { # [default] Resolvable , Unresolvable { key : Option < String > , } , }
    };
}

Resolvability!()