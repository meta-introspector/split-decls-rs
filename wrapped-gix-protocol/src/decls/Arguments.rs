macro_rules! deps {
    () => {
        Feature!();
    };
}

macro_rules! Arguments {
    () => {
        deps!();
        # [doc = " The arguments passed to a server command."] # [derive (Debug)] pub struct Arguments { # [doc = " The active features/capabilities of the fetch invocation"] # [cfg (any (feature = "async-client" , feature = "blocking-client"))] features : Vec < crate :: command :: Feature > , args : Vec < BString > , haves : Vec < BString > , filter : bool , shallow : bool , deepen_since : bool , deepen_not : bool , deepen_relative : bool , ref_in_want : bool , supports_include_tag : bool , features_for_first_want : Option < Vec < String > > , # [cfg (any (feature = "async-client" , feature = "blocking-client"))] version : gix_transport :: Protocol , # [cfg (any (feature = "async-client" , feature = "blocking-client"))] trace : bool , }
    };
}

Arguments!();