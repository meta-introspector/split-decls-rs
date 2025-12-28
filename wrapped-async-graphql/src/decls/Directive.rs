macro_rules! Directive {
    () => {
        # [doc = " A GraphQL directive"] # [derive (Debug , Clone)] pub struct Directive { name : String , args : IndexMap < String , Value > , }
    };
}

Directive!();