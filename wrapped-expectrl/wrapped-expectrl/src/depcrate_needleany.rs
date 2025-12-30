// Generated macro for Any (struct)
macro_rules! Depcrate_needleAny {
() => {
// Module: crate::needle
// Provides: {"Any"}
// Dependencies: {}
# [doc = " Any matches uses all provided lookups and returns a match"] # [doc = " from a first successfull match."] # [doc = ""] # [doc = " It does checks lookups in order they were provided."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run,ignore"] # [doc = " use expectrl::{spawn, Any};"] # [doc = ""] # [doc = " let mut p = spawn(\"cat\").unwrap();"] # [doc = " p.expect(Any([\"we\", \"are\", \"here\"])).unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " To be able to combine different types of lookups you can call [Any::boxed]."] # [doc = ""] # [doc = " ```no_run,ignore"] # [doc = " use expectrl::{spawn, Any, NBytes};"] # [doc = ""] # [doc = " let mut p = spawn(\"cat\").unwrap();"] # [doc = " p.expect(Any::boxed(vec![Box::new(\"we\"), Box::new(NBytes(3))])).unwrap();"] # [doc = " ```"] # [derive (Debug)] pub struct Any < I > (pub I) ;
};
}
