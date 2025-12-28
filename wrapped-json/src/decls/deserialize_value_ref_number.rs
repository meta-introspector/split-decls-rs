macro_rules! deps {
    () => {
        Result!();
        Value!();
        Error!();
        Number!();
    };
}

macro_rules! deserialize_value_ref_number {
    () => {
        deps!();
        macro_rules ! deserialize_value_ref_number { ($ method : ident) => { # [cfg (not (feature = "arbitrary_precision"))] fn $ method < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor <'de >, { match self { Value :: Number (n) => n . deserialize_any (visitor) , _ => Err (self . invalid_type (& visitor)) , } } # [cfg (feature = "arbitrary_precision")] fn $ method < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor <'de >, { match self { Value :: Number (n) => n .$ method (visitor) , _ => self . deserialize_any (visitor) , } } } ; }
    };
}

deserialize_value_ref_number!();