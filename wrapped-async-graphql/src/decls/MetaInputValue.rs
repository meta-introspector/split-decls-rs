macro_rules! deps {
    () => {
        MetaDirectiveInvocation!();
        Field!();
        MetaVisibleFn!();
        Deprecation!();
    };
}

macro_rules! MetaInputValue {
    () => {
        deps!();
        # [doc = " Input value metadata"] # [derive (Clone)] pub struct MetaInputValue { # [doc = " The name of the input value"] pub name : String , # [doc = " The description of the input value"] pub description : Option < String > , # [doc = " The type of the input value"] pub ty : String , # [doc = " Field deprecation"] pub deprecation : Deprecation , # [doc = " The default value of the input value"] pub default_value : Option < String > , # [doc = " A function that uses to check if the input value should be exported to"] # [doc = " schemas"] pub visible : Option < MetaVisibleFn > , # [doc = " Indicate that an input object is not accessible from a supergraph when"] # [doc = " using Apollo Federation"] pub inaccessible : bool , # [doc = " Arbitrary string metadata that will be propagated to the supergraph when"] # [doc = " using Apollo Federation. This attribute is repeatable"] pub tags : Vec < String > , # [doc = " Indicate that an input object is secret"] pub is_secret : bool , # [doc = " Custom directive invocations"] pub directive_invocations : Vec < MetaDirectiveInvocation > , }
    };
}

MetaInputValue!();