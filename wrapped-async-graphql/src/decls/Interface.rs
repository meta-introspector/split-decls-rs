macro_rules! deps {
    () => {
        Directive!();
        InterfaceField!();
    };
}

macro_rules! Interface {
    () => {
        deps!();
        # [doc = " A GraphQL interface type"] # [derive (Debug)] pub struct Interface { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) fields : IndexMap < String , InterfaceField > , pub (crate) implements : IndexSet < String > , keys : Vec < String > , extends : bool , inaccessible : bool , tags : Vec < String > , pub (crate) directives : Vec < Directive > , requires_scopes : Vec < String > , }
    };
}

Interface!();