macro_rules! deps {
    () => {
        MetaInputValue!();
        CacheControl!();
        MetaDirectiveInvocation!();
        ComputeComplexityFn!();
        MetaVisibleFn!();
        Field!();
        Deprecation!();
    };
}

macro_rules! MetaField {
    () => {
        deps!();
        # [doc = " Field metadata"] # [derive (Clone)] pub struct MetaField { # [doc = " The name of the field"] pub name : String , # [doc = " The description of the field"] pub description : Option < String > , # [doc = " The arguments of the field"] pub args : IndexMap < String , MetaInputValue > , # [doc = " The type of the field"] pub ty : String , # [doc = " Field deprecation"] pub deprecation : Deprecation , # [doc = " Used to create HTTP `Cache-Control` header"] pub cache_control : CacheControl , # [doc = " Mark a field as owned by another service. This allows service A to use"] # [doc = " fields from service B while also knowing at runtime the types of that"] # [doc = " field."] pub external : bool , # [doc = " Annotate the required input fieldset from a base type for a resolver. It"] # [doc = " is used to develop a query plan where the required fields may not be"] # [doc = " needed by the client, but the service may need additional information"] # [doc = " from other services."] pub requires : Option < String > , # [doc = " Annotate the expected returned fieldset from a field on a base type that"] # [doc = " is guaranteed to be selectable by the gateway."] pub provides : Option < String > , # [doc = " A function that uses to check if the field should be exported to"] # [doc = " schemas"] pub visible : Option < MetaVisibleFn > , # [doc = " Indicate that an object type's field is allowed to be resolved by"] # [doc = " multiple subgraphs"] pub shareable : bool , # [doc = " Indicate that an object is not accessible from a supergraph when using"] # [doc = " Apollo Federation"] pub inaccessible : bool , # [doc = " Arbitrary string metadata that will be propagated to the supergraph when"] # [doc = " using Apollo Federation. This attribute is repeatable"] pub tags : Vec < String > , # [doc = " Mark the field as overriding a field currently present on another"] # [doc = " subgraph. It is used to migrate fields between subgraphs."] pub override_from : Option < String > , # [doc = " A constant or function to get the complexity"] pub compute_complexity : Option < ComputeComplexityFn > , # [doc = " Custom directive invocations"] pub directive_invocations : Vec < MetaDirectiveInvocation > , # [doc = " Indicates to composition that the target element is accessible only to"] # [doc = " the authenticated supergraph users with the appropriate JWT scopes"] # [doc = " when using Apollo Federation."] pub requires_scopes : Vec < String > , }
    };
}

MetaField!()