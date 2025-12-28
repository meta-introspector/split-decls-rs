macro_rules! BareFunctionType {
    () => {
        # [doc = " The `<bare-function-type>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <bare-function-type> ::= <signature type>+"] # [doc = "      # types are possible return type, then parameter types"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct BareFunctionType (Vec < TypeHandle >) ;
    };
}

BareFunctionType!()